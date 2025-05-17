use crate::Control;
use iced::mouse::Cursor;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::path::lyon_path::geom::euclid::{Transform2D, Vector2D};
use iced::widget::canvas::{Frame, Geometry, Path, Program};
use iced::{Point, Rectangle, Renderer, Size, Theme, Vector};
use maccel_core::inputspeed::{read_input_speed, setup_input_speed_reader};
use maccel_core::{ContextRef, persist::ParamStore, sensitivity};

#[derive(Debug)]
pub struct Graph<PS: ParamStore> {
    context: ContextRef<PS>,
}

impl<PS: ParamStore> Graph<PS> {
    pub const AXIS_BOUNDS: Size = Size::new(80., 3.);
    pub const fn graph_area(size: Size) -> Rectangle {
        const ORIGIN_MARGIN: f32 = 40.;
        const EDGE_MARGIN: f32 = 20.;
        Rectangle {
            x: ORIGIN_MARGIN,
            y: size.height - ORIGIN_MARGIN,
            width: size.width - ORIGIN_MARGIN - EDGE_MARGIN,
            height: -size.height + ORIGIN_MARGIN + EDGE_MARGIN,
        }
    }

    pub fn new(context: ContextRef<PS>) -> Self {
        setup_input_speed_reader();
        Graph { context }
    }
    fn build_plots(&self, x_bld: &mut Builder, y_bld: &mut Builder, bounds: Rectangle) {
        const STEP_SZ: f32 = 0.25;

        let mut v = bounds.x;
        let (x_sens, y_sens) = sensitivity(
            v as f64,
            self.context.get().current_mode,
            &self.context.get().params_snapshot(),
        );
        let (x_sens, y_sens) = (x_sens as f32, y_sens as f32);
        x_bld.move_to(Point { x: v, y: x_sens });
        y_bld.move_to(Point { x: v, y: y_sens });
        v += STEP_SZ;

        while v <= bounds.x + bounds.width {
            let (x_sens, y_sens) = sensitivity(
                v as f64,
                self.context.get().current_mode,
                &self.context.get().params_snapshot(),
            );
            let (x_sens, y_sens) = (x_sens as f32, y_sens as f32);
            if (bounds.y..=bounds.y + bounds.height).contains(&x_sens) {
                x_bld.line_to(Point { x: v, y: x_sens });
            } else {
                x_bld.move_to(Point { x: v, y: x_sens });
            }
            if (bounds.y..=bounds.y + bounds.height).contains(&y_sens) {
                y_bld.line_to(Point { x: v, y: y_sens });
            } else {
                y_bld.move_to(Point { x: v, y: y_sens });
            }
            v += STEP_SZ;
        }
    }
}

impl<M, PS: ParamStore> Program<M> for Graph<PS> {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let axes = Graph::<PS>::AXIS_BOUNDS;
        let area = Graph::<PS>::graph_area(bounds.size());
        let theme = crate::GraphTheme::default(axes, area);
        let transform = Transform2D::scale(area.width / axes.width, area.height / axes.height)
            .then_translate(Vector2D::new(area.x, area.y));

        let mut frame = Frame::new(renderer, bounds.size());

        let input_speed = (read_input_speed() as f32).clamp(0., axes.width);
        let (h_speedo, v_speedo) = {
            let mut h_bld = Builder::new();
            let mut v_bld = Builder::new();
            self.build_plots(
                &mut h_bld,
                &mut v_bld,
                Rectangle {
                    x: 0.,
                    y: 0.,
                    width: input_speed,
                    height: axes.height,
                },
            );
            h_bld.line_to(Point {
                x: input_speed,
                y: 0.,
            });
            v_bld.line_to(Point {
                x: input_speed,
                y: 0.,
            });
            h_bld.line_to(Point::ORIGIN);
            v_bld.line_to(Point::ORIGIN);
            h_bld.close();
            v_bld.close();
            (
                h_bld.build().transform(&transform),
                v_bld.build().transform(&transform),
            )
        };
        frame.fill(&h_speedo, theme.h_speedo_fill);
        frame.fill(&v_speedo, theme.v_speedo_fill);

        let (h_plot, v_plot) = {
            let mut h_bld = Builder::new();
            let mut v_bld = Builder::new();
            self.build_plots(&mut h_bld, &mut v_bld, Rectangle::with_size(axes));
            (
                h_bld.build().transform(&transform),
                v_bld.build().transform(&transform),
            )
        };
        frame.stroke(&h_plot, theme.h_plot_stroke);
        frame.stroke(&v_plot, theme.v_plot_stroke);

        let x_axis = Path::line(
            area.position() + Vector::new(-10., 0.),
            area.position() + Vector::new(area.width, 0.),
        );
        let y_axis = Path::line(
            area.position() + Vector::new(0., 10.),
            area.position() + Vector::new(0., area.height),
        );
        frame.stroke(&x_axis, theme.x_axis_stroke);
        frame.stroke(&y_axis, theme.y_axis_stroke);

        let controls = Control::controls(self.context.clone());
        let mut x_labels = (1..=axes.width as u32 / 10)
            .map(|u| u as f32 * 10.)
            .collect::<Vec<_>>();
        let mut y_labels = (1..=axes.height as u32 * 2)
            .map(|u| u as f32 / 2.)
            .collect::<Vec<_>>();
        x_labels.extend(controls.iter().map(|c| c.location().x));
        y_labels.extend(controls.iter().map(|c| c.location().y));
        x_labels.retain(|f| (0. ..=axes.width).contains(f));
        y_labels.retain(|f| (0. ..=axes.height).contains(f));
        x_labels.sort_by(f32::total_cmp);
        y_labels.sort_by(f32::total_cmp);
        x_labels.dedup();
        y_labels.dedup();
        for f in x_labels {
            frame.fill_text((theme.x_label_text)(f));
        }
        for f in y_labels {
            frame.fill_text((theme.y_label_text)(f));
        }

        vec![frame.into_geometry()]
    }
}
