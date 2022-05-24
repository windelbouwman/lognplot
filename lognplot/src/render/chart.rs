//! Functionality to emit a plot to a canvas.

use super::canvas::{HorizontalAnchor, VerticalAnchor};
use super::transform;
use super::Canvas;
use super::{ChartLayout, ChartOptions};
use crate::chart::{Chart, Cursor, Curve};
use crate::geometry::Point;
use crate::style::Color;
use crate::time::TimeStamp;
use crate::tsdb::observations::{
    Aggregation, CountMetrics, Observation, ProfileEvent, Sample, SampleMetrics, Text,
};
use crate::tsdb::{QueryResult, RangeQueryResult};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use superslice::Ext;

/// Draw the given chart onto the canvas!
pub fn draw_chart<C>(
    chart: &Chart,
    canvas: &mut C,
    layout: &mut ChartLayout,
    options: &ChartOptions,
) where
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

type CurveData = Option<QueryResult>;

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
    layout: &'a mut ChartLayout,

    // Parameters:
    options: &'a ChartOptions,

    curve_data_cache: HashMap<String, Rc<CurveData>>,

    text_track_y: f64,
}

impl<'a, C> ChartRenderer<'a, C>
where
    C: Canvas,
{
    pub fn new(
        chart: &'a Chart,
        canvas: &'a mut C,
        layout: &'a mut ChartLayout,
        options: &'a ChartOptions,
    ) -> Self {
        ChartRenderer {
            chart,
            canvas,
            layout,
            options,
            curve_data_cache: HashMap::new(),
            text_track_y: 0.0,
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
            let top_center = Point::new(self.layout.width / 2.0, self.options.padding);
            self.canvas.print_text(
                &top_center,
                HorizontalAnchor::Middle,
                VerticalAnchor::Top,
                title,
            );
        }
    }

    /// Draw x and y axis with tick markers.
    fn draw_axis(&mut self) {
        self.layout.layout(&self.options);

        if let Some(title) = &self.chart.title {
            self.layout.title_height = self.canvas.text_size(title).height + self.options.padding;
        } else {
            self.layout.title_height = 0.0;
        }

        let n_x_ticks = (self.layout.plot_width as usize / PIXELS_PER_X_TICK).max(2);
        let (prefix, x_ticks) = self.chart.x_axis.calc_date_tiks(n_x_ticks);

        let n_y_ticks = (self.layout.plot_height as usize / PIXELS_PER_Y_TICK).max(2);
        let y_ticks = self.chart.y_axis.calc_tiks(n_y_ticks);

        // Now we have the ticks, calculate space taken by the ticks and re-layout!
        let y_labels_max_width = y_ticks
            .iter()
            .map(|t| self.canvas.text_size(&t.1).width)
            .fold(1.0, |a, b| if a > b { a } else { b });
        self.layout.y_axis_legend_width =
            y_labels_max_width + self.options.tick_size * 2.0 + self.options.padding;
        // println!("Y axis width: {}, ticks={:?}", self.layout.y_axis_legend_width, y_ticks);
        let x_labels_max_height = x_ticks
            .iter()
            .map(|t| self.canvas.text_size(&t.1).height)
            .fold(1.0, |a, b| if a > b { a } else { b });
        self.layout.x_axis_legend_height =
            x_labels_max_height + self.options.tick_size * 2.0 + self.options.padding;
        self.layout.info_bar_height = self.canvas.text_size("X").height;
        // println!("X axis height: {}, ticks={:?}", self.layout.x_axis_legend_height, x_ticks);
        self.layout.layout(&self.options);

        // Now we are ready to paint!
        self.canvas.set_pen(Color::white(), 1.0);
        self.canvas.fill_rect(
            self.layout.plot_left,
            self.layout.plot_top,
            self.layout.plot_width,
            self.layout.plot_height,
        );

        let x_ticks: Vec<(TimeStamp, String)> = x_ticks
            .into_iter()
            .map(|(x, s)| (TimeStamp::new(x), s))
            .collect();
        self.draw_x_axis(prefix, &x_ticks);

        self.draw_y_axis(&y_ticks);

        // Draw grid
        self.draw_grid(&x_ticks, &y_ticks);
    }

    // X axis:
    fn draw_x_axis(&mut self, prefix: Option<String>, x_ticks: &[(TimeStamp, String)]) {
        self.canvas.set_pen(Color::black(), 1.0);
        self.canvas.set_line_width(1.0);

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

        let y = self.layout.plot_bottom;
        // let baseline = vec![
        //     Point::new(self.layout.plot_left, y),
        //     Point::new(self.layout.plot_right, y),
        // ];

        // self.canvas.draw_line(&baseline);

        for (p, label) in x_ticks.iter() {
            let x = self.x_domain_to_pixel(p);
            let p1 = Point::new(x, y + self.options.tick_size * 2.0);
            let p2 = Point::new(x, y);
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
        self.canvas.set_line_width(1.0);

        if let Some(title) = &self.chart.y_axis.label {
            let p = Point::new(10.0, self.layout.height / 2.0);
            self.canvas
                .print_text(&p, HorizontalAnchor::Left, VerticalAnchor::Middle, title);
        }

        let x = self.layout.plot_left;
        // let baseline = vec![
        //     Point::new(x, self.layout.plot_top),
        //     Point::new(x, self.layout.plot_bottom),
        // ];

        // self.canvas.draw_line(&baseline);

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
            if self.chart.x_axis.contains(&cursor.0) {
                self.draw_cursor_line(&cursor.0, true);
                // TBD: how usable / visually helpful is this??
                self.draw_cursor_values(cursor);
            }
        }

        if let Some(cursor) = &self.chart.cursor1 {
            self.draw_cursor_line(cursor, false);
        }

        if let Some(cursor) = &self.chart.cursor2 {
            self.draw_cursor_line(cursor, false);
        }

        if let (Some(cur1), Some(cur2)) = (&self.chart.cursor1, &self.chart.cursor2) {
            let dt: f64 = (cur1.amount - cur2.amount).abs();
            let freq = if dt > 1e-10 {
                let freq: f64 = 1.0 / dt;
                format!("{} Hz", freq)
            } else {
                "inf".to_owned()
            };
            let text = format!("dt = {} s, F = {}", dt, freq);
            let p = Point::new(
                self.layout.width - self.options.padding,
                self.layout.height - self.options.padding,
            );
            self.canvas
                .print_text(&p, HorizontalAnchor::Right, VerticalAnchor::Bottom, &text);
        }
    }

    /// Draw the cursor as a line on the data.
    fn draw_cursor_line(&mut self, cursor: &TimeStamp, draw_label: bool) {
        let x = self.x_domain_to_pixel(&cursor);
        let top = Point::new(x, self.layout.plot_top);
        let bottom = Point::new(x, self.layout.plot_bottom);
        let points = vec![top, bottom];

        self.canvas.set_pen(Color::black(), 1.0);
        self.canvas.set_line_width(2.0);
        self.canvas.draw_line(&points);

        if draw_label {
            // cursor label:
            let label_top = Point::new(x, self.layout.height - self.options.padding);
            let label = self.chart.x_axis.get_cursor_label(&cursor);
            self.canvas.print_text(
                &label_top,
                HorizontalAnchor::Middle,
                VerticalAnchor::Bottom,
                &label,
            );
        }
    }

    fn get_cursor_values(
        &self,
        cursor: &Cursor,
    ) -> Vec<(Option<(TimeStamp, f64)>, Vec<String>, Color)> {
        let mut values = vec![];
        for curve in &self.chart.curves {
            if let Some(curve_data) = self.query_curve_data(&curve).borrow() {
                match curve_data {
                    QueryResult::Value(value_data) => match value_data {
                        RangeQueryResult::Aggregations(aggregations) => {
                            if let Some(a) = find_closest_aggregation(&aggregations, &cursor.0) {
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
                                values.push((Some((ts, mean)), labels, curve.color()));
                            }
                        }
                        RangeQueryResult::Observations(observations) => {
                            if let Some(o) = find_closest_observation(&observations, &cursor.0) {
                                let ts = o.timestamp.clone();
                                let value = o.value.value;
                                let label = format!("{}", value);
                                values.push((Some((ts, value)), vec![label], curve.color()));
                            }
                        }
                    },
                    QueryResult::Text(text_data) => {
                        match text_data {
                            RangeQueryResult::Aggregations(_aggregations) => {
                                // TODO
                            }
                            RangeQueryResult::Observations(observations) => {
                                if let Some(o) = find_last_observation(&observations, &cursor.0) {
                                    let label = o.value.text.clone();
                                    values.push((None, vec![label], curve.color()));
                                }
                            }
                        }
                    }
                    QueryResult::Profile(_profile_data) => {}
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
            for (marker, _, color) in values.iter() {
                if let Some((ts, value)) = marker {
                    let x = self.x_domain_to_pixel(&ts);
                    let y = self.y_domain_to_pixel(*value);
                    let p1 = Point::new(x, y);
                    self.canvas.set_pen(color.clone(), 1.0);
                    self.canvas.draw_circle(&p1, 8.0);
                }
            }

            let padding = 3.0;
            let mut background_width: f64 = 0.0;
            let mut background_height: f64 = 0.0;
            let square_size: f64 = self.canvas.text_size("X").height;

            for (_, labels, _) in values.iter() {
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
            for (_, labels, color) in values {
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
                    QueryResult::Value(value_data) => match value_data {
                        RangeQueryResult::Aggregations(aggregations) => {
                            self.draw_aggregations(aggregations, color);
                        }
                        RangeQueryResult::Observations(observations) => {
                            let draw_markers =
                                observations.len() < pixels / (PIXELS_PER_AGGREGATION * 5);
                            self.draw_observations(observations, color, draw_markers);
                        }
                    },
                    QueryResult::Text(text_data) => match text_data {
                        RangeQueryResult::Aggregations(aggregations) => {
                            self.draw_text_aggregations(aggregations, color);
                        }
                        RangeQueryResult::Observations(observations) => {
                            self.draw_text_observations(observations, color);
                        }
                    },
                    QueryResult::Profile(profile_data) => {
                        match profile_data {
                            RangeQueryResult::Observations(observations) => {
                                self.draw_profile_observations(observations, color);
                            }
                            RangeQueryResult::Aggregations(_aggregations) => {
                                // TODO!
                            }
                        }
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

    /// Draw a single series of observed textual events.
    fn draw_text_observations(&mut self, observations: &[Observation<Text>], color: Color) {
        let mut texts = vec![];

        for observation in observations {
            let observation_x = self.x_domain_to_pixel(&observation.timestamp);
            texts.push((observation_x, observation.value.text.clone()));
        }

        self.draw_texts_in_track(texts, color);
    }

    fn draw_text_aggregations(
        &mut self,
        aggregations: &[Aggregation<Text, CountMetrics>],
        color: Color,
    ) {
        let mut texts = vec![];

        for aggregation in aggregations {
            let observation_x = self.x_domain_to_pixel(&aggregation.timespan.start);
            texts.push((observation_x, aggregation.metrics().count.to_string()));
        }

        self.draw_texts_in_track(texts, color);
    }

    /// Helper function to draw a sequence of texts between two lines.
    /// Also proceed to the next text track slot.
    fn draw_texts_in_track(&mut self, texts: Vec<(f64, String)>, color: Color) {
        self.canvas.set_pen(color, 1.0);
        self.canvas.set_line_width(2.0);

        let text_height = self.canvas.text_size("X").height;
        let padding = 5.0;

        let track_y = self.layout.plot_bottom - 20.0 - self.text_track_y;
        self.text_track_y += text_height + padding * 3.0;

        let track_left = self.layout.plot_left;
        let track_right = self.layout.plot_right;
        let track_top = track_y - (text_height / 2.0) - padding;
        let track_bottom = track_y + (text_height / 2.0) + padding;

        // Draw top line:
        self.canvas.draw_line(&[
            Point::new(track_left, track_top),
            Point::new(track_right, track_top),
        ]);

        // Draw bottom line:
        self.canvas.draw_line(&[
            Point::new(track_left, track_bottom),
            Point::new(track_right, track_bottom),
        ]);

        let end_markers = texts
            .iter()
            .skip(1)
            .map(|t| t.0)
            .chain(std::iter::once(track_right));

        // Draw text boxes:
        for ((x, text), end_x) in texts.iter().zip(end_markers) {
            self.canvas
                .draw_line(&[Point::new(*x, track_top), Point::new(*x, track_bottom)]);

            let max_text_width: f64 = end_x - *x - padding * 2.0;

            let point = Point::new(x + padding, track_y);

            self.draw_dotted_text(
                &point,
                HorizontalAnchor::Left,
                VerticalAnchor::Middle,
                text,
                max_text_width,
            );
        }
    }

    /// Draw a text and trim the text length if it does not fit within the given width.
    fn draw_dotted_text(
        &mut self,
        point: &Point,
        horizontal_anchor: HorizontalAnchor,
        vertical_anchor: VerticalAnchor,
        text: &str,
        max_text_width: f64,
    ) {
        let text_width = self.canvas.text_size(text).width;
        if text_width < max_text_width {
            // we can fit the text!
            self.canvas
                .print_text(&point, horizontal_anchor, vertical_anchor, text);
        } else {
            // trimming required..
            let dots = "...";
            let dots_width = self.canvas.text_size(dots).width;
            // Do not draw text in too small regions
            // so only draw text when we can fit the 3 dots.
            if dots_width < max_text_width {
                // Shrink text until it fits
                let chars = text.chars();
                let mut prefix = String::new();
                let mut dotted_text: String = dots.to_owned();
                for ch in chars {
                    prefix.push(ch);
                    let new_dotted_text = prefix.clone() + dots;
                    let new_dotted_text_width = self.canvas.text_size(&new_dotted_text).width;
                    if new_dotted_text_width > max_text_width {
                        break;
                    }
                    dotted_text = new_dotted_text;
                }
                self.canvas
                    .print_text(&point, horizontal_anchor, vertical_anchor, &dotted_text);
            }
        }
    }

    fn draw_profile_observations(
        &mut self,
        _observations: &[Observation<ProfileEvent>],
        color: Color,
    ) {
        self.canvas.set_pen(color, 1.0);
        self.canvas.set_line_width(2.0);

        // TODO!
    }

    /// Transform x-value to pixel/point location.
    fn x_domain_to_pixel(&self, t: &TimeStamp) -> f64 {
        transform::x_domain_to_pixel(t, &self.chart.x_axis, &self.layout)
    }

    /// Convert a y value into a proper pixel y value.
    fn y_domain_to_pixel(&self, y: f64) -> f64 {
        transform::y_domain_to_pixel(y, &self.chart.y_axis, &self.layout)
    }
}

/// Find the last observation at the given time in a sorted list of observations.
fn find_last_observation<'o, V>(
    observations: &'o [Observation<V>],
    t: &TimeStamp,
) -> Option<&'o Observation<V>> {
    find_last(observations, t, |o| o.timestamp.clone())
}

// Find the last value before the given timestamp.
fn find_last<'o, T, F>(things: &'o [T], ts: &TimeStamp, f: F) -> Option<&'o T>
where
    F: Fn(&T) -> TimeStamp,
{
    let idx = things.lower_bound_by(|a| f(a).partial_cmp(ts).unwrap());
    if (idx == 0) || (idx > things.len()) {
        None
    } else {
        Some(&things[idx - 1])
    }
}

/// Find the closest observation in a sorted list of observations.
fn find_closest_observation<'o, V>(
    observations: &'o [Observation<V>],
    t: &TimeStamp,
) -> Option<&'o Observation<V>> {
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
