//! Chart functionality!

use super::axis::{calc_tiks, Axis};
use super::curve::Curve;
use crate::canvas::{Canvas, Color};
use crate::geometry::Point;

/// A single 2D-chart
pub struct Chart {
    /// An optional title for the plot
    title: Option<String>,

    x_axis: Axis,
    y_axis: Axis,

    /// To show grid or not.
    grid: bool,

    /// The curves in the plot
    curves: Vec<Curve>,
}

impl Default for Chart {
    fn default() -> Self {
        Chart {
            title: None,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            grid: true,
            curves: vec![],
        }
    }
}

impl Chart {
    /// Set the title of the chart!
    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn set_xlabel(&mut self, label: &str) {
        self.x_axis.label = Some(label.to_string());
    }

    pub fn set_ylabel(&mut self, label: &str) {
        self.y_axis.label = Some(label.to_string());
    }

    /// Drop a new curve into the mix!
    pub fn add_curve(&mut self, curve: Curve) {
        self.curves.push(curve);
    }

    /// Adjust scale ranges so we fit all data in view.
    pub fn autoscale(&mut self) {
        // self.curves
        self.x_axis.set_limits(0.0, 1000.0);
        self.y_axis.set_limits(0.0, 1000.0);
    }

    /// Draw the whole chart
    pub fn draw(&self, canvas: &mut dyn Canvas) {
        self.draw_box(canvas);
        self.draw_axis(canvas);
        self.draw_lines(canvas);

        // Print title of chart:
        canvas.set_pen(Color::black());
        let top_center = Point::new(50.0, 0.0);
        if let Some(title) = &self.title {
            canvas.print_text(&top_center, title);
        }
    }

    /// Draw x and y axis with tick markers.
    fn draw_axis(&self, canvas: &mut dyn Canvas) {
        // X axis:
        if let Some(title) = &self.x_axis.label {
            let p = Point::new(500.0, 990.0);
            canvas.print_text(&p, title);
        }

        let x_ticks = calc_tiks(&self.x_axis);

        canvas.set_pen(Color::black());
        for (p, label) in x_ticks.iter() {
            let p1 = Point::new(*p, 10.0);
            let p2 = Point::new(*p, 15.0);
            let p3 = Point::new(*p, 20.0);
            canvas.print_text(&p1, label);
            canvas.draw_line(&p2, &p3);
        }

        // y axis:
        if let Some(title) = &self.y_axis.label {
            let p = Point::new(10.0, 600.0);
            canvas.print_text(&p, title);
        }

        let y_ticks = calc_tiks(&self.y_axis);
        canvas.set_pen(Color::gray());
        for (p, label) in y_ticks.iter() {
            let p1 = Point::new(10.0, *p);
            let p2 = Point::new(15.0, *p);
            let p3 = Point::new(20.0, *p);
            canvas.print_text(&p1, label);
            canvas.draw_line(&p2, &p3);
        }

        // Draw grid
        if self.grid {
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
    fn draw_box(&self, canvas: &mut dyn Canvas) {
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

    fn draw_lines(&self, canvas: &mut dyn Canvas) {
        for curve in &self.curves {
            trace!("Plotting curve {:?}", curve);

            canvas.set_pen(curve.color());

            // Create pairs:
            let pairs = curve.points.iter().zip(curve.points.iter().skip(1));
            for (p1, p2) in pairs {
                canvas.draw_line(p1, p2);
            }
        }
    }
}
