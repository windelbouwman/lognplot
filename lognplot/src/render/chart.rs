//! Functionality to emit a plot to a canvas.

use super::canvas::{HorizontalAnchor, VerticalAnchor};
use super::transform;
use super::Canvas;
use super::{ChartLayout, ChartOptions};
use crate::chart::{Chart, Cursor, Curve};
use crate::geometry::Point;
use crate::style::Color;
use crate::time::TimeStamp;
use crate::tsdb::{Aggregation, Observation, RangeQueryResult, Sample, SampleMetrics};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use superslice::Ext;

/// Draw the given chart onto the canvas!
pub fn draw_chart<C>(chart: &Chart, canvas: &mut C, layout: &ChartLayout, options: &ChartOptions)
where
    C: Canvas,
{
    let mut renderer = ChartRenderer::new(chart, canvas, layout, options);
    renderer.draw();
}

/// How much space there appears between x axis ticks
const PIXELS_PER_X_TICK: usize = 100;

/// How much pixels approximately to have between y ticks.
const PIXELS_PER_Y_TICK: usize = 60;

/// Divide the width of the plot by this value, and draw at least that many data points.
const PIXELS_PER_AGGREGATION: usize = 5;

type CurveData = Option<RangeQueryResult<Sample, SampleMetrics>>;

/// This struct will be able to render the chart onto a canvas.
struct ChartRenderer<'a, C>
where
    C: Canvas,
{
    // The chart to render.
    chart: &'a Chart,

    // The canvas to render to!
    canvas: &'a mut C,

    // Layout:
    layout: &'a ChartLayout,

    // Parameters:
    options: &'a ChartOptions,

    curve_data_cache: HashMap<String, Rc<CurveData>>,
}

impl<'a, C> ChartRenderer<'a, C>
where
    C: Canvas,
{
    pub fn new(
        chart: &'a Chart,
        canvas: &'a mut C,
        layout: &'a ChartLayout,
        options: &'a ChartOptions,
    ) -> Self {
        ChartRenderer {
            chart,
            canvas,
            layout,
            options,
            curve_data_cache: HashMap::new(),
        }
    }

    fn draw(&mut self) {
        self.fetch_curve_data();
        self.draw_axis();
        self.draw_box();
        self.draw_curves();
        self.draw_cursor();
        self.draw_title();
        self.draw_legend();
    }

    fn draw_legend(&mut self) {
        let x = self.layout.plot_left + 10.0;
        let mut y = self.layout.plot_top + 10.0;

        // Grab height of capital x as text height:
        let text_height = self.canvas.text_size("X").height;
        let square_size = text_height;
        let dy = text_height * 1.3;

        for curve in &self.chart.curves {
            let name = curve.name();
            let color = curve.color();
            self.canvas.set_pen(color, 1.0);
            self.canvas
                .fill_rect(x, y - square_size / 2.0, square_size, square_size);
            let p = Point::new(x + dy, y);
            self.canvas.set_pen(Color::black(), 1.0);
            self.canvas
                .print_text(&p, HorizontalAnchor::Left, VerticalAnchor::Middle, &name);
            y += dy;
        }
    }

    /// Print title of chart
    fn draw_title(&mut self) {
        if let Some(title) = &self.chart.title {
            self.canvas.set_pen(Color::black(), 1.0);
            let top_center = Point::new(50.0, 0.0);
            self.canvas.print_text(
                &top_center,
                HorizontalAnchor::Middle,
                VerticalAnchor::Bottom,
                title,
            );
        }
    }

    /// Draw x and y axis with tick markers.
    fn draw_axis(&mut self) {
        let n_x_ticks = (self.layout.plot_width as usize / PIXELS_PER_X_TICK).max(2);
        let (prefix, ticks) = self.chart.x_axis.calc_date_tiks(n_x_ticks);

        let x_ticks: Vec<(TimeStamp, String)> = ticks
            .into_iter()
            .map(|(x, s)| (TimeStamp::new(x), s))
            .collect();
        self.draw_x_axis(prefix, &x_ticks);

        let n_y_ticks = (self.layout.plot_height as usize / PIXELS_PER_Y_TICK).max(2);
        let y_ticks = self.chart.y_axis.calc_tiks(n_y_ticks);

        self.draw_y_axis(&y_ticks);

        // Draw grid
        self.draw_grid(&x_ticks, &y_ticks);
    }

    // X axis:
    fn draw_x_axis(&mut self, prefix: Option<String>, x_ticks: &[(TimeStamp, String)]) {
        self.canvas.set_pen(Color::black(), 1.0);
        self.canvas.set_line_width(2.0);

        if let Some(title) = &self.chart.x_axis.label {
            let p = Point::new(self.layout.width / 2.0, self.layout.height - 2.0);
            self.canvas
                .print_text(&p, HorizontalAnchor::Middle, VerticalAnchor::Bottom, title);
        }

        if let Some(prefix) = prefix {
            let p = Point::new(
                self.options.padding,
                self.layout.height - self.options.padding,
            );
            self.canvas
                .print_text(&p, HorizontalAnchor::Left, VerticalAnchor::Bottom, &prefix);
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
            let horizontal_anchor = HorizontalAnchor::Middle;

            self.canvas
                .print_text(&p1, horizontal_anchor, VerticalAnchor::Top, label);
            let line = vec![p2, p3];
            self.canvas.draw_line(&line);
        }
    }

    // y axis:
    fn draw_y_axis(&mut self, y_ticks: &[(f64, String)]) {
        self.canvas.set_pen(Color::black(), 1.0);
        self.canvas.set_line_width(2.0);

        if let Some(title) = &self.chart.y_axis.label {
            let p = Point::new(10.0, self.layout.height / 2.0);
            self.canvas
                .print_text(&p, HorizontalAnchor::Left, VerticalAnchor::Middle, title);
        }

        let x = self.layout.plot_left - self.options.tick_size;
        let baseline = vec![
            Point::new(x, self.layout.plot_top),
            Point::new(x, self.layout.plot_bottom),
        ];

        self.canvas.draw_line(&baseline);

        for (p, label) in y_ticks.iter() {
            let y = self.y_domain_to_pixel(*p);
            let p1 = Point::new(x - self.options.tick_size * 2.0, y);
            let p2 = Point::new(x, y);
            let p3 = Point::new(x - self.options.tick_size, y);
            self.canvas
                .print_text(&p1, HorizontalAnchor::Right, VerticalAnchor::Middle, label);
            let line = vec![p2, p3];
            self.canvas.draw_line(&line);
        }
    }

    fn draw_grid(&mut self, x_ticks: &[(TimeStamp, String)], y_ticks: &[(f64, String)]) {
        self.canvas.set_pen(Color::gray(), 1.0);
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
        self.canvas.set_pen(Color::black(), 1.0);
        self.canvas.set_line_width(2.0);
        let outline = vec![top_left, top_right, bottom_right, bottom_left];
        self.canvas.draw_polygon(&outline);
    }

    /// Draw a cursor and some values along it.
    fn draw_cursor(&mut self) {
        if let Some(cursor) = &self.chart.cursor {
            self.draw_cursor_line(cursor);
            // TBD: how usable / visually helpful is this??
            self.draw_cursor_values(cursor);
        }
    }

    /// Draw the cursor as a line on the data.
    fn draw_cursor_line(&mut self, cursor: &Cursor) {
        let x = self.x_domain_to_pixel(&cursor.0);
        let top = Point::new(x, self.layout.plot_top);
        let bottom = Point::new(x, self.layout.plot_bottom);
        let points = vec![top, bottom];

        self.canvas.set_pen(Color::black(), 1.0);
        self.canvas.set_line_width(2.0);
        self.canvas.draw_line(&points);
    }

    fn get_cursor_values(&self, cursor: &Cursor) -> Vec<(TimeStamp, f64, Vec<String>, Color)> {
        let mut values = vec![];
        for curve in &self.chart.curves {
            if let Some(curve_data) = self.query_curve_data(&curve).borrow() {
                let value = match curve_data {
                    RangeQueryResult::Aggregations(aggregations) => {
                        find_closest_aggregation(&aggregations, &cursor.0).map(|a| {
                            let ts = a.timespan.middle_timestamp();
                            let metrics = a.metrics();
                            let min = metrics.min;
                            let mean = metrics.mean();
                            let max = metrics.max;
                            let labels = vec![
                                format!("mean={}", mean),
                                format!("min={}", min),
                                format!("max={}", max),
                            ];
                            (ts, mean, labels, curve.color())
                        })
                    }
                    RangeQueryResult::Observations(observations) => {
                        find_closest_observation(&observations, &cursor.0).map(|o| {
                            let ts = o.timestamp.clone();
                            let value = o.value.value;
                            let label = format!("{}", value);
                            (ts, value, vec![label], curve.color())
                        })
                    }
                };

                if let Some(value) = value {
                    values.push(value);
                }
            }
        }
        values
    }

    /// Draw values around cursor.
    fn draw_cursor_values(&mut self, cursor: &Cursor) {
        let values = self.get_cursor_values(cursor);
        if !values.is_empty() {
            // Draw circle markers:
            for (ts, value, _, color) in values.iter() {
                let x = self.x_domain_to_pixel(&ts);
                let y = self.y_domain_to_pixel(*value);
                let p1 = Point::new(x, y);
                self.canvas.set_pen(color.clone(), 1.0);
                self.canvas.draw_circle(&p1, 8.0);
            }

            let padding = 3.0;
            let mut background_width: f64 = 0.0;
            let mut background_height: f64 = 0.0;
            let square_size: f64 = self.canvas.text_size("X").height;

            for (_, _, labels, _) in values.iter() {
                for label in labels {
                    let text_size = self.canvas.text_size(&label);

                    if text_size.width > background_width {
                        background_width = text_size.width;
                    }
                    background_height += text_size.height + padding;
                }
            }

            let x = self.x_domain_to_pixel(&cursor.0) + padding;
            let mut y = self.y_domain_to_pixel(cursor.1);

            background_width += padding * 3.0 + square_size;
            background_height += padding;

            self.canvas.set_pen(Color::white(), 1.0);
            self.canvas
                .fill_rect(x, y, background_width, background_height);
            self.canvas.set_pen(Color::black(), 1.0);
            self.canvas
                .draw_rect(x, y, background_width, background_height);

            // Draw background rectangle and labels
            // let background_width = values.iter().map().max()
            for (_, _, labels, color) in values {
                // Draw color:
                self.canvas.set_pen(color, 1.0);
                self.canvas
                    .fill_rect(x + padding, y + padding, square_size, square_size);

                // Draw labels:
                self.canvas.set_pen(Color::black(), 1.0);
                for label in labels {
                    let p = Point::new(
                        x + square_size + padding * 2.0,
                        y + padding + square_size / 2.0,
                    );
                    self.canvas.print_text(
                        &p,
                        HorizontalAnchor::Left,
                        VerticalAnchor::Middle,
                        &label,
                    );
                    y += square_size + padding;
                }
            }
        }
    }

    /// Draw the actual curves!
    fn draw_curves(&mut self) {
        let pixels: usize = self.layout.plot_width as usize;

        for curve in &self.chart.curves {
            // trace!("Plotting curve {:?}", curve);

            let color = curve.color();
            if let Some(curve_data) = self.query_curve_data(&curve).borrow() {
                match curve_data {
                    RangeQueryResult::Aggregations(aggregations) => {
                        self.draw_aggregations(aggregations, color);
                    }
                    RangeQueryResult::Observations(observations) => {
                        let draw_markers =
                            observations.len() < pixels / (PIXELS_PER_AGGREGATION * 5);
                        self.draw_observations(observations, color, draw_markers);
                    }
                }
            }
        }
    }

    /// Fetch curve data from backing data store.
    fn fetch_curve_data(&mut self) {
        let timespan = self.chart.x_axis.timespan();
        let pixels: usize = self.layout.plot_width as usize;
        let point_count = pixels / PIXELS_PER_AGGREGATION;
        for curve in &self.chart.curves {
            let data = curve.query(&timespan, point_count);
            self.curve_data_cache.insert(curve.name(), Rc::new(data));
        }
    }

    fn query_curve_data(&self, curve: &Curve) -> Rc<CurveData> {
        self.curve_data_cache[&curve.name()].clone()
    }

    /// Draw single observations.
    fn draw_observations(
        &mut self,
        observations: &[Observation<Sample>],
        color: Color,
        draw_markers: bool,
    ) {
        let points: Vec<Point> = observations
            .iter()
            .map(|o| {
                Point::new(
                    self.x_domain_to_pixel(&o.timestamp),
                    self.y_domain_to_pixel(o.value.value),
                )
            })
            .collect();
        trace!("Drawing {} points", points.len());

        self.canvas.set_pen(color, 1.0);
        self.canvas.set_line_width(2.0);
        self.canvas.draw_line(&points);

        // Draw markers as small solid square dots
        // Idea from pulseview (sigrok application)
        if draw_markers {
            for point in points {
                self.canvas
                    .fill_rect(point.x() - 4.0, point.y() - 4.0, 8.0, 8.0);
            }
        }
    }

    /// Draw aggregated values
    fn draw_aggregations(
        &mut self,
        aggregations: &[Aggregation<Sample, SampleMetrics>],
        color: Color,
    ) {
        if aggregations.is_empty() {
            return;
        }

        // Create polygon from metrics:
        let mut top_line: Vec<Point> = vec![];
        let mut mean_line: Vec<Point> = vec![];
        let mut bottom_line: Vec<Point> = vec![];
        let mut stddev_high_line: Vec<Point> = vec![];
        let mut stddev_low_line: Vec<Point> = vec![];

        let first_point = {
            let aggregation = aggregations.first().unwrap();
            let x = self.x_domain_to_pixel(&aggregation.timespan.start);
            let y = self.y_domain_to_pixel(aggregation.metrics().first);
            Point::new(x, y)
        };

        let last_point = {
            let aggregation = aggregations.last().unwrap();
            let x = self.x_domain_to_pixel(&aggregation.timespan.end);
            let y = self.y_domain_to_pixel(aggregation.metrics().last);
            Point::new(x, y)
        };

        mean_line.push(first_point);

        for aggregation in aggregations {
            let y_max_value = aggregation.metrics().max;
            let y_min_value = aggregation.metrics().min;
            let y_max = self.y_domain_to_pixel(y_max_value);
            let y_min = self.y_domain_to_pixel(y_min_value);
            let mean = aggregation.metrics().mean();
            let stddev = aggregation.metrics().stddev();
            let y_mean = self.y_domain_to_pixel(mean);

            let visually_nice = true;
            let y_stddev_high_value = if visually_nice {
                // Can the mean + stddev be higher than max? --> YES IT CAN :)
                // Clip the mean + stddev to min and max values.
                y_max_value.min(mean + stddev)
            } else {
                // scientifically more correct, but less nice visually:
                mean + stddev
            };
            let y_stddev_high = self.y_domain_to_pixel(y_stddev_high_value);

            let y_stddev_low_value = if visually_nice {
                y_min_value.max(mean - stddev)
            } else {
                mean - stddev
            };
            let y_stddev_low = self.y_domain_to_pixel(y_stddev_low_value);

            // TBD: what is a good visualization of aggregations?
            // blocks or not?
            let draw_block = false;
            if draw_block {
                let x1 = self.x_domain_to_pixel(&aggregation.timespan.start);
                top_line.push(Point::new(x1, y_max));
                bottom_line.push(Point::new(x1, y_min));
                mean_line.push(Point::new(x1, y_mean));
                stddev_high_line.push(Point::new(x1, y_stddev_high));
                stddev_low_line.push(Point::new(x1, y_stddev_low));

                let x2 = self.x_domain_to_pixel(&aggregation.timespan.end);
                top_line.push(Point::new(x2, y_max));
                bottom_line.push(Point::new(x2, y_min));
                mean_line.push(Point::new(x2, y_mean));
                stddev_high_line.push(Point::new(x2, y_stddev_high));
                stddev_low_line.push(Point::new(x2, y_stddev_low));
            } else {
                let x3 = self.x_domain_to_pixel(&aggregation.timespan.middle_timestamp());

                top_line.push(Point::new(x3, y_max));
                bottom_line.push(Point::new(x3, y_min));
                mean_line.push(Point::new(x3, y_mean));
                stddev_high_line.push(Point::new(x3, y_stddev_high));
                stddev_low_line.push(Point::new(x3, y_stddev_low));
            }
        }

        mean_line.push(last_point);

        let min_max_poly_points: Vec<Point> = {
            let mut points = vec![];
            points.push(first_point);
            points.extend(top_line);
            points.push(last_point);
            bottom_line.reverse();
            points.extend(bottom_line);
            points
        };

        let stddev_poly_points: Vec<Point> = {
            let mut points = vec![];
            points.push(first_point);
            points.extend(stddev_high_line);
            points.push(last_point);
            stddev_low_line.reverse();
            points.extend(stddev_low_line);
            points
        };

        trace!("Polygon with {} points", min_max_poly_points.len());
        trace!("Mean line with {} points", mean_line.len());

        // Draw the contour and mean line.
        self.canvas.set_pen(color.clone(), 0.3);
        self.canvas.fill_polygon(&min_max_poly_points);

        // std deviation outline:
        self.canvas.set_pen(color.clone(), 0.6);
        self.canvas.fill_polygon(&stddev_poly_points);

        // mean line
        self.canvas.set_pen(color, 1.0);
        self.canvas.set_line_width(2.0);
        self.canvas.draw_line(&mean_line);
    }

    /// Transform x-value to pixel/point location.
    fn x_domain_to_pixel(&self, t: &TimeStamp) -> f64 {
        transform::x_domain_to_pixel(t, &self.chart.x_axis, self.layout)
    }

    /// Convert a y value into a proper pixel y value.
    fn y_domain_to_pixel(&self, y: f64) -> f64 {
        transform::y_domain_to_pixel(y, &self.chart.y_axis, self.layout)
    }
}

/// Find the closest observation in a sorted list of observations.
fn find_closest_observation<'o>(
    observations: &'o [Observation<Sample>],
    t: &TimeStamp,
) -> Option<&'o Observation<Sample>> {
    find_closest(observations, t, |o| o.timestamp.clone())
}

/// Find the aggregation which is closest to a certain timestamp
/// in a sorted list of aggregations.
fn find_closest_aggregation<'o>(
    aggregations: &'o [Aggregation<Sample, SampleMetrics>],
    t: &TimeStamp,
) -> Option<&'o Aggregation<Sample, SampleMetrics>> {
    find_closest(aggregations, t, |a| a.timespan.middle_timestamp())
}

/// Find the thing which is closest to the given timestamp.
/// The function f must be given to turn a thing into a timestamp.
fn find_closest<'o, T, F>(things: &'o [T], ts: &TimeStamp, f: F) -> Option<&'o T>
where
    F: Fn(&T) -> TimeStamp,
{
    let idx = things.lower_bound_by(|a| f(a).partial_cmp(ts).unwrap());
    if (idx == 0) || (idx >= things.len()) {
        None
    } else {
        // Compare two aggregations
        let t1 = &things[idx - 1];
        let t2 = &things[idx];
        if f(t1).distance(ts) < f(t2).distance(ts) {
            Some(t1)
        } else {
            Some(t2)
        }
    }
}
