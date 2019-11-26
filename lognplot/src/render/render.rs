//! Functionality to emit a plot to a canvas.

use super::Canvas;
use super::{ChartLayout, ChartOptions};
use crate::chart::Chart;
use crate::geometry::{Point, Size};
use crate::style::Color;
use crate::time::TimeStamp;
use crate::tsdb::{Aggregation, Observation, RangeQueryResult, Sample, SampleMetrics};

/// Draw the given chart onto the canvas!
pub fn draw_chart(chart: &Chart, canvas: &mut dyn Canvas, size: Size) {
    let mut renderer = ChartRenderer::new(chart, canvas, size);
    renderer.draw();
}

/// This struct will be able to render the chart onto a canvas.
struct ChartRenderer<'a> {
    chart: &'a Chart,
    canvas: &'a mut dyn Canvas,

    // Layout:
    layout: ChartLayout,
    // Parameters:
    options: ChartOptions,
}

impl<'a> ChartRenderer<'a> {
    pub fn new(chart: &'a Chart, canvas: &'a mut dyn Canvas, size: Size) -> Self {
        let options = ChartOptions::default();
        let layout = ChartLayout::new(size);
        ChartRenderer {
            chart,
            canvas,
            layout,
            options,
        }
    }

    fn draw(&mut self) {
        self.layout.layout(&self.options);
        self.draw_axis();
        self.draw_box();
        self.draw_curves();

        // Print title of chart:
        self.canvas.set_pen(Color::black());
        let top_center = Point::new(50.0, 0.0);
        if let Some(title) = &self.chart.title {
            self.canvas.print_text(&top_center, title);
        }
    }

    /// Draw x and y axis with tick markers.
    fn draw_axis(&mut self) {
        let n_x_ticks = self.layout.plot_width as usize / 50;
        let x_ticks: Vec<(TimeStamp, String)> = self
            .chart
            .x_axis
            .calc_tiks(n_x_ticks)
            .into_iter()
            .map(|(x, s)| (TimeStamp::new(x), s))
            .collect();
        self.draw_x_axis(&x_ticks);

        let n_y_ticks = self.layout.plot_height as usize / 50;
        let y_ticks = self.chart.y_axis.calc_tiks(n_y_ticks);

        self.draw_y_axis(&y_ticks);

        // Draw grid
        self.draw_grid(&x_ticks, &y_ticks);
    }

    // X axis:
    fn draw_x_axis(&mut self, x_ticks: &[(TimeStamp, String)]) {
        self.canvas.set_pen(Color::black());
        self.canvas.set_line_width(2.0);

        if let Some(title) = &self.chart.x_axis.label {
            let p = Point::new(self.layout.width / 2.0, self.layout.height - 10.0);
            self.canvas.print_text(&p, title);
        }

        let y = self.layout.plot_bottom + self.options.tick_size;
        let baseline = vec![
            Point::new(self.layout.plot_left, y),
            Point::new(self.layout.plot_right, y),
        ];

        self.canvas.draw_line(&baseline);

        for (p, label) in x_ticks.iter() {
            let x = self.x_domain_to_pixel(p);
            let p1 = Point::new(x, y + self.options.tick_size + 5.0);
            let p2 = Point::new(x, self.layout.plot_bottom + self.options.tick_size);
            let p3 = Point::new(x, y + self.options.tick_size);
            self.canvas.print_text(&p1, label);
            let line = vec![p2, p3];
            self.canvas.draw_line(&line);
        }
    }

    // y axis:
    fn draw_y_axis(&mut self, y_ticks: &[(f64, String)]) {
        self.canvas.set_pen(Color::black());
        self.canvas.set_line_width(2.0);

        if let Some(title) = &self.chart.y_axis.label {
            let p = Point::new(10.0, self.layout.height / 2.0);
            self.canvas.print_text(&p, title);
        }

        let x = self.layout.plot_left - self.options.tick_size;
        let baseline = vec![
            Point::new(x, self.layout.plot_top),
            Point::new(x, self.layout.plot_bottom),
        ];

        self.canvas.draw_line(&baseline);

        for (p, label) in y_ticks.iter() {
            let y = self.y_domain_to_pixel(*p);
            let p1 = Point::new(x - self.options.tick_size * 2.0 - 30.0, y);
            let p2 = Point::new(x, y);
            let p3 = Point::new(x - self.options.tick_size, y);
            self.canvas.print_text(&p1, label);
            let line = vec![p2, p3];
            self.canvas.draw_line(&line);
        }
    }

    fn draw_grid(&mut self, x_ticks: &[(TimeStamp, String)], y_ticks: &[(f64, String)]) {
        self.canvas.set_pen(Color::gray());
        self.canvas.set_line_width(1.0);

        if self.chart.grid {
            // vertical grid lines:
            for (p, _) in x_ticks.iter() {
                let x = self.x_domain_to_pixel(p);
                let p1 = Point::new(x, self.layout.plot_top);
                let p2 = Point::new(x, self.layout.plot_bottom);
                let line = vec![p1, p2];
                self.canvas.draw_line(&line);
            }

            // horizontal grid lines:
            for (p, _) in y_ticks.iter() {
                let y = self.y_domain_to_pixel(*p);
                let p1 = Point::new(self.layout.plot_left, y);
                let p2 = Point::new(self.layout.plot_right, y);
                let line = vec![p1, p2];
                self.canvas.draw_line(&line);
            }
        }
    }

    /// Draw chart box
    fn draw_box(&mut self) {
        let top_left = Point::new(self.layout.plot_left, self.layout.plot_top);
        let bottom_left = Point::new(self.layout.plot_left, self.layout.plot_bottom);
        let top_right = Point::new(self.layout.plot_right, self.layout.plot_top);
        let bottom_right = Point::new(self.layout.plot_right, self.layout.plot_bottom);

        // Draw four lines:
        self.canvas.set_pen(Color::black());
        self.canvas.set_line_width(2.0);
        let outline = vec![top_left, top_right, bottom_right, bottom_left];
        self.canvas.draw_polygon(&outline);
    }

    /// Draw the actual curves!
    fn draw_curves(&mut self) {
        let timespan = self.chart.x_axis.timespan();
        let point_count = (self.layout.plot_width as usize) / 40;

        for curve in &self.chart.curves {
            // trace!("Plotting curve {:?}", curve);

            self.canvas.set_pen(curve.color());
            self.canvas.set_line_width(2.0);

            // Create pairs:
            let curve_data = curve.query(&timespan, point_count);
            match curve_data {
                RangeQueryResult::Aggregations(aggregations) => {
                    self.draw_aggregations(aggregations);
                }
                RangeQueryResult::Observations(observations) => {
                    self.draw_observations(observations);
                }
            }
        }
    }

    /// Draw single observations.
    fn draw_observations(&mut self, observations: Vec<Observation<Sample>>) {
        let points: Vec<Point> = observations
            .into_iter()
            .map(|o| {
                Point::new(
                    self.x_domain_to_pixel(&o.timestamp),
                    self.y_domain_to_pixel(o.value.value),
                )
            })
            .collect();
        trace!("Drawing {} points", points.len());

        self.canvas.draw_line(&points);
    }

    /// Draw aggregated values
    fn draw_aggregations(&mut self, aggregations: Vec<Aggregation<Sample, SampleMetrics>>) {
        // Create polygon from metrics:
        let mut top_line: Vec<Point> = vec![];
        let mut mean_line: Vec<Point> = vec![];
        let mut bottom_line: Vec<Point> = vec![];

        for aggregation in aggregations {
            let x1 = self.x_domain_to_pixel(&aggregation.timespan.start);
            let x2 = self.x_domain_to_pixel(&aggregation.timespan.end);
            let y_max = self.y_domain_to_pixel(aggregation.metrics().max);
            let y_min = self.y_domain_to_pixel(aggregation.metrics().min);
            let y_mean = self.y_domain_to_pixel(aggregation.metrics().mean());
            top_line.push(Point::new(x1, y_max));
            top_line.push(Point::new(x2, y_max));
            bottom_line.push(Point::new(x1, y_min));
            bottom_line.push(Point::new(x2, y_min));
            mean_line.push(Point::new(x1, y_mean));
            mean_line.push(Point::new(x2, y_mean));
        }

        bottom_line.reverse();
        let mut poly_points: Vec<Point> = top_line;
        poly_points.extend(bottom_line);
        trace!("Polygon with {} points", poly_points.len());
        trace!("Mean line with {} points", mean_line.len());

        // Draw the contour and mean line.
        self.canvas.set_pen(Color::new(100, 100, 0));
        self.canvas.fill_polygon(&poly_points);
        self.canvas.set_pen(Color::new(200, 0, 0));
        self.canvas.draw_line(&mean_line);
    }

    /// Transform x-value to pixel/point location.
    fn x_domain_to_pixel(&self, t: &TimeStamp) -> f64 {
        let x = t.amount;
        let domain = self.chart.x_axis.domain();
        let a = (self.layout.plot_width) / domain;
        a * (x - self.chart.x_axis.begin()) + self.layout.plot_left
    }

    /// Convert a y value into a proper pixel y value.
    fn y_domain_to_pixel(&self, y: f64) -> f64 {
        let domain = self.chart.y_axis.domain();
        let a = self.layout.plot_height / domain;
        self.layout.plot_bottom - a * (y - self.chart.y_axis.begin())
    }
}
