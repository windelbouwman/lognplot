//! Functionality to emit a plot to a canvas.

use super::Chart;
use quartzcanvas::geometry::Point;
use quartzcanvas::style::Color;
use quartzcanvas::{Canvas, CanvasDrawAble};

impl CanvasDrawAble for Chart {
    /// Draw the whole chart
    fn draw(&self, canvas: &mut dyn Canvas) {
        draw_chart(self, canvas);
    }
}

/// Draw the given chart onto the canvas!
fn draw_chart(chart: &Chart, canvas: &mut dyn Canvas) {
    let mut renderer = ChartRenderer::new(chart, canvas);
    renderer.draw();
}

/// This struct will be able to render the chart onto a canvas.
struct ChartRenderer<'a> {
    chart: &'a Chart,
    canvas: &'a mut dyn Canvas,

    // Layout:
    width: f64,
    height: f64,
    plot_top: f64,
    plot_left: f64,
    plot_bottom: f64,
    plot_right: f64,
    plot_width: f64,
    plot_height: f64,

    // Parameters:
    options: ChartOptions,
}

struct ChartOptions {
    tick_size: f64,
    padding: f64,
}

impl Default for ChartOptions {
    fn default() -> Self {
        ChartOptions {
            tick_size: 7.0,
            padding: 10.0,
        }
    }
}

impl<'a> ChartRenderer<'a> {
    pub fn new(chart: &'a Chart, canvas: &'a mut dyn Canvas) -> Self {
        let options = ChartOptions::default();
        ChartRenderer {
            chart,
            canvas,

            // TODO: casowary?
            width: 600.0,
            height: 600.0,
            plot_top: 0.0,
            plot_left: 0.0,
            plot_bottom: 0.0,
            plot_right: 0.0,
            plot_width: 0.0,
            plot_height: 0.0,

            // parameters
            options,
        }
    }

    fn draw(&mut self) {
        self.layout();
        self.draw_axis();
        self.draw_box();
        self.draw_lines();

        // Print title of chart:
        self.canvas.set_pen(Color::black());
        let top_center = Point::new(50.0, 0.0);
        if let Some(title) = &self.chart.title {
            self.canvas.print_text(&top_center, title);
        }
    }

    fn layout(&mut self) {
        self.plot_top = self.options.padding;
        self.plot_left = 50.0;
        self.plot_bottom = self.height - 50.0;
        self.plot_right = self.width - self.options.padding;
        self.plot_height = self.plot_right - self.plot_left;
        self.plot_width = self.plot_bottom - self.plot_top;
    }

    /// Draw x and y axis with tick markers.
    fn draw_axis(&mut self) {
        let n_x_ticks = self.plot_width as usize / 50;
        let x_ticks = self.chart.x_axis.calc_tiks(n_x_ticks);
        self.draw_x_axis(&x_ticks);

        let n_y_ticks = self.plot_height as usize / 50;
        let y_ticks = self.chart.y_axis.calc_tiks(n_y_ticks);

        self.draw_y_axis(&y_ticks);

        // Draw grid
        self.draw_grid(&x_ticks, &y_ticks);
    }

    // X axis:
    fn draw_x_axis(&mut self, x_ticks: &[(f64, String)]) {
        self.canvas.set_pen(Color::black());

        if let Some(title) = &self.chart.x_axis.label {
            let p = Point::new(self.height / 2.0, self.height - 10.0);
            self.canvas.print_text(&p, title);
        }

        self.canvas.draw_line(
            &Point::new(self.plot_left, self.plot_bottom + self.options.tick_size),
            &Point::new(self.plot_right, self.plot_bottom + self.options.tick_size),
        );

        for (p, label) in x_ticks.iter() {
            let x = self.x_domain_to_pixel(*p);
            let p1 = Point::new(
                x,
                self.plot_bottom + self.options.tick_size + self.options.tick_size + 5.0,
            );
            let p2 = Point::new(x, self.plot_bottom + self.options.tick_size);
            let p3 = Point::new(
                x,
                self.plot_bottom + self.options.tick_size + self.options.tick_size,
            );
            self.canvas.print_text(&p1, label);
            self.canvas.draw_line(&p2, &p3);
        }
    }

    // y axis:
    fn draw_y_axis(&mut self, y_ticks: &[(f64, String)]) {
        self.canvas.set_pen(Color::black());

        if let Some(title) = &self.chart.y_axis.label {
            let p = Point::new(10.0, self.height / 2.0);
            self.canvas.print_text(&p, title);
        }

        self.canvas.draw_line(
            &Point::new(self.plot_left - self.options.tick_size, self.plot_top),
            &Point::new(self.plot_left - self.options.tick_size, self.plot_bottom),
        );

        for (p, label) in y_ticks.iter() {
            let y = self.y_domain_to_pixel(*p);
            let p1 = Point::new(self.plot_left - self.options.tick_size * 2.0 - 30.0, y);
            let p2 = Point::new(self.plot_left - self.options.tick_size, y);
            let p3 = Point::new(self.plot_left - self.options.tick_size * 2.0, y);
            self.canvas.print_text(&p1, label);
            self.canvas.draw_line(&p2, &p3);
        }
    }

    fn draw_grid(&mut self, x_ticks: &[(f64, String)], y_ticks: &[(f64, String)]) {
        if self.chart.grid {
            // vertical grid lines:
            for (p, _) in x_ticks.iter() {
                let x = self.x_domain_to_pixel(*p);
                let p1 = Point::new(x, self.plot_top);
                let p2 = Point::new(x, self.plot_bottom);
                self.canvas.draw_line(&p1, &p2);
            }

            // horizontal grid lines:
            for (p, _) in y_ticks.iter() {
                let y = self.y_domain_to_pixel(*p);
                let p1 = Point::new(self.plot_left, y);
                let p2 = Point::new(self.plot_right, y);
                self.canvas.draw_line(&p1, &p2);
            }
        }
    }

    /// Draw chart box
    fn draw_box(&mut self) {
        let top_left = Point::new(self.plot_left, self.plot_top);
        let bottom_left = Point::new(self.plot_left, self.plot_bottom);
        let top_right = Point::new(self.plot_right, self.plot_top);
        let bottom_right = Point::new(self.plot_right, self.plot_bottom);

        // Draw four lines:
        self.canvas.set_pen(Color::black());
        self.canvas.draw_line(&top_left, &top_right);
        self.canvas.draw_line(&top_left, &bottom_left);
        self.canvas.draw_line(&bottom_left, &bottom_right);
        self.canvas.draw_line(&top_right, &bottom_right);
    }

    /// Draw the actual curves!
    fn draw_lines(&mut self) {
        for curve in &self.chart.curves {
            // trace!("Plotting curve {:?}", curve);

            self.canvas.set_pen(curve.color());

            // Create pairs:
            let points: Vec<Point> = curve
                .get_points()
                .into_iter()
                .map(|p| Point::new(self.x_domain_to_pixel(p.x()), self.y_domain_to_pixel(p.y())))
                .collect();

            let pairs = points.iter().zip(points.iter().skip(1));
            for (p1, p2) in pairs {
                self.canvas.draw_line(p1, p2);
            }
        }
    }

    /// Transform x-value to pixel/point location.
    fn x_domain_to_pixel(&self, x: f64) -> f64 {
        let a = (self.plot_width) / (self.chart.x_axis.domain());
        a * (x - self.chart.x_axis.begin()) + self.plot_left
    }

    fn y_domain_to_pixel(&self, y: f64) -> f64 {
        let a = self.plot_height / self.chart.y_axis.domain();
        self.plot_bottom - a * (y - self.chart.y_axis.begin())
    }
}
