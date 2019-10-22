//! Functionality to emit a plot to a canvas.

use super::canvas::{Canvas, CanvasDrawAble};
use crate::geometry::Point;
use crate::plot::Chart;
use crate::style::Color;

impl CanvasDrawAble for Chart {
    /// Draw the whole chart
    fn draw(&self, canvas: &mut dyn Canvas) {
        draw_box(self, canvas);
        draw_axis(self, canvas);
        draw_lines(self, canvas);

        // Print title of chart:
        canvas.set_pen(Color::black());
        let top_center = Point::new(50.0, 0.0);
        if let Some(title) = &self.title {
            canvas.print_text(&top_center, title);
        }
    }
}

/// Draw x and y axis with tick markers.
fn draw_axis(chart: &Chart, canvas: &mut dyn Canvas) {
    // X axis:
    if let Some(title) = &chart.x_axis.label {
        let p = Point::new(500.0, 990.0);
        canvas.print_text(&p, title);
    }

    let x_ticks = chart.x_axis.calc_tiks();

    canvas.set_pen(Color::black());
    for (p, label) in x_ticks.iter() {
        let p1 = Point::new(*p, 10.0);
        let p2 = Point::new(*p, 15.0);
        let p3 = Point::new(*p, 20.0);
        canvas.print_text(&p1, label);
        canvas.draw_line(&p2, &p3);
    }

    // y axis:
    if let Some(title) = &chart.y_axis.label {
        let p = Point::new(10.0, 600.0);
        canvas.print_text(&p, title);
    }

    let y_ticks = chart.y_axis.calc_tiks();
    canvas.set_pen(Color::gray());
    for (p, label) in y_ticks.iter() {
        let p1 = Point::new(10.0, *p);
        let p2 = Point::new(15.0, *p);
        let p3 = Point::new(20.0, *p);
        canvas.print_text(&p1, label);
        canvas.draw_line(&p2, &p3);
    }

    // Draw grid
    if chart.grid {
        // vertical grid lines:
        for (p, _) in x_ticks.iter() {
            let p1 = Point::new(*p, 20.0);
            let p2 = Point::new(*p, 980.0);
            canvas.draw_line(&p1, &p2);
        }

        // horizontal grid lines:
        for (p, _) in y_ticks.iter() {
            let p1 = Point::new(20.0, *p);
            let p2 = Point::new(980.0, *p);
            canvas.draw_line(&p1, &p2);
        }
    }
}

/// Draw chart box
fn draw_box(_chart: &Chart, canvas: &mut dyn Canvas) {
    let top_left = Point::new(20.0, 20.0);
    let bottom_left = Point::new(20.0, 980.0);
    let top_right = Point::new(980.0, 20.0);
    let bottom_right = Point::new(980.0, 980.0);

    // Draw four lines:
    canvas.set_pen(Color::black());
    canvas.draw_line(&top_left, &top_right);
    canvas.draw_line(&top_left, &bottom_left);
    canvas.draw_line(&bottom_left, &bottom_right);
    canvas.draw_line(&top_right, &bottom_right);
}

fn draw_lines(chart: &Chart, canvas: &mut dyn Canvas) {
    for curve in &chart.curves {
        trace!("Plotting curve {:?}", curve);

        canvas.set_pen(curve.color());

        // Create pairs:
        let points = curve.get_points();
        let pairs = points.iter().zip(points.iter().skip(1));
        for (p1, p2) in pairs {
            canvas.draw_line(p1, p2);
        }
    }
}
