//! GUI state for a single chart.
//!
//!
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::session::DashBoardItem;
use crate::state::GuiStateHandle;
use crate::time_tracker::TimeTracker;
use lognplot::chart::{Chart, Curve, CurveData};
use lognplot::geometry::Size;
use lognplot::render::{draw_chart, CairoCanvas, ChartLayout, ChartOptions};
use lognplot::render::{x_pixel_to_domain, x_pixels_to_domain, y_pixel_to_domain};
use lognplot::time::{TimeSpan, TimeStamp};
use lognplot::tracer::{AnyTracer, Tracer};
use lognplot::tsdb::DataChangeEvent;
use lognplot::tsdb::TsDbHandle;
use std::sync::Arc;

/// category10 color wheel
///
/// See also: https://matplotlib.org/users/dflt_style_changes.html#colors-in-default-property-cycle
const CATEGORY10_COLORS: &[&str] = &[
    "#1F77B4", "#FF7F0E", "#2CA02C", "#D62728", "#9467BD", "#8C564B", "#E377C2", "#7F7F7F",
    "#BCBD22", "#17BECF",
];

pub type ChartStateHandle = Rc<RefCell<ChartState>>;

pub struct ChartState {
    chart: Chart,
    chart_options: ChartOptions,
    chart_layout: ChartLayout,
    db: TsDbHandle,
    app_state: GuiStateHandle,
    color_wheel: Vec<String>,
    color_index: usize,
    tailing: Option<f64>,
    perf_tracer: Arc<AnyTracer>,
    drag: Option<(f64, f64)>,
    draw_area: gtk::DrawingArea,
    id: String,
    time_estimator: TimeTracker,
}

impl ChartState {
    pub fn new(
        db: TsDbHandle,
        perf_tracer: Arc<AnyTracer>,
        app_state: GuiStateHandle,
        draw_area: gtk::DrawingArea,
        id: &str,
    ) -> Self {
        let mut chart = Chart::default();
        chart.set_title(id);
        let color_wheel: Vec<String> = CATEGORY10_COLORS.iter().map(|s| (*s).to_string()).collect();

        info!("Chart id: {}", id);

        ChartState {
            chart,
            chart_options: ChartOptions::default(),
            chart_layout: ChartLayout::new(Size::new(250.0, 250.0)),
            db: db.clone(),
            app_state,
            color_wheel,
            color_index: 0,
            tailing: None,
            perf_tracer: perf_tracer.clone(),
            drag: None,
            draw_area,
            id: id.to_owned(),
            time_estimator: TimeTracker::new(perf_tracer, id),
        }
    }

    pub fn into_handle(self) -> ChartStateHandle {
        Rc::new(RefCell::new(self))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn add_curve(&mut self, name: &str) {
        // self.chart.add_curve(Curve::new());
        if !self.chart.has_signal(name) {
            let tsdb_data = CurveData::trace(name, self.db.clone());
            let color = self.next_color();
            let curve2 = Curve::new(tsdb_data, &color);

            self.chart.add_curve(curve2);
            self.chart.autoscale();
            self.repaint();
        } else {
            info!("Signal {} is already shown", name);
        }
    }

    fn next_color(&mut self) -> String {
        let color = self.color_wheel[self.color_index].clone();
        self.color_index += 1;
        if self.color_index >= self.color_wheel.len() {
            self.color_index = 0;
        }
        color
    }

    pub fn clear_curves(&mut self) {
        debug!("Kill all signals!");
        self.disable_tailing();
        self.chart.clear_curves();
        self.repaint();
    }

    pub fn set_cursor1(&mut self) {
        debug!("set cursor 1!");
        self.chart.set_cursor1();
        self.repaint();
    }

    pub fn set_cursor2(&mut self) {
        debug!("set cursor 2!");
        self.chart.set_cursor2();
        self.repaint();
    }

    pub fn get_session_item(&self) -> DashBoardItem {
        (&self.chart).into()
    }

    pub fn set_session_item(&mut self, item: &DashBoardItem) {
        if let DashBoardItem::Graph { curves } = item {
            self.clear_curves();
            for curve in curves {
                self.add_curve(curve);
            }
        }
    }

    /// Handle data change event from database.
    pub fn handle_event(&mut self, event: &DataChangeEvent) {
        // Check if we must update the chart:
        let update = event.delete_all
            || event
                .changed_signals
                .iter()
                .any(|n| self.chart.has_signal(n));
        if update {
            if let Some(last_time) = self.chart.get_last_timestamp() {
                self.time_estimator.update(last_time.amount);
            }
            self.repaint();
        }
    }

    fn repaint(&self) {
        self.draw_area.queue_draw();
    }

    /// Initial drag action of the mouse
    pub fn start_drag(&mut self, x: f64, y: f64) {
        debug!("Drag start! {}, {} ", x, y);
        self.disable_tailing();
        self.drag = Some((x, y));
    }

    /// Update drag of the mouse
    pub fn move_drag(&mut self, x: f64, y: f64) {
        self.disable_tailing();
        if let Some((prev_x, prev_y)) = self.drag {
            let dx = x - prev_x;
            let dy = y - prev_y;
            self.do_drag(dx, dy);
        }
        self.drag = Some((x, y));
    }

    /// Drag the plot by the given amount.
    fn do_drag(&mut self, dx: f64, dy: f64) {
        debug!("Drag! {}, {} ", dx, dy);

        let amount = x_pixels_to_domain(&self.chart_layout, &self.chart.x_axis, dx);

        self.chart.pan_horizontal_absolute(-amount);
        self.handle_x_axis_change();
    }

    pub fn zoom_fit(&mut self) {
        debug!("Autoscale!");
        self.disable_tailing();
        self.chart.autoscale();
        self.repaint();
    }

    pub fn zoom_in_vertical(&mut self) {
        debug!("Zoom in vertical");
        self.zoom_vertical(0.1);
    }

    pub fn zoom_out_vertical(&mut self) {
        debug!("Zoom out vertical");
        self.zoom_vertical(-0.1);
    }

    fn zoom_vertical(&mut self, amount: f64) {
        self.disable_tailing();
        self.chart.zoom_vertical(amount);
        self.repaint();
    }

    pub fn zoom_in_horizontal(&mut self, around: Option<f64>) {
        debug!("Zoom in horizontal");
        self.zoom_horizontal(-0.1, around);
    }

    pub fn zoom_out_horizontal(&mut self, around: Option<f64>) {
        debug!("Zoom out horizontal");
        self.zoom_horizontal(0.1, around);
    }

    fn zoom_horizontal(&mut self, amount: f64, around: Option<f64>) {
        let around =
            around.map(|pixel| x_pixel_to_domain(pixel, &self.chart.x_axis, &self.chart_layout));
        self.disable_tailing();
        self.chart.zoom_horizontal(amount, around);
        self.handle_x_axis_change();
    }

    pub fn set_cursor(&mut self, loc: Option<(f64, f64)>) {
        if let Some((pixel_x, pixel_y)) = loc {
            let timestamp = x_pixel_to_domain(pixel_x, &self.chart.x_axis, &self.chart_layout);
            let value = y_pixel_to_domain(pixel_y, &self.chart.y_axis, &self.chart_layout);
            let timestamp = TimeStamp::new(timestamp);
            self.chart.cursor = Some((timestamp, value));
        } else {
            self.chart.cursor = None;
        }

        // Sync cursor to other plots:
        self.app_state.borrow().sync_cursor(&self);

        self.repaint();
    }

    pub fn pan_left(&mut self) {
        debug!("pan left!");
        self.disable_tailing();
        self.chart.pan_horizontal_relative(-0.1);
        self.handle_x_axis_change();
    }

    pub fn pan_right(&mut self) {
        debug!("Pan right!");
        self.disable_tailing();
        self.chart.pan_horizontal_relative(0.1);
        self.handle_x_axis_change();
    }

    pub fn pan_up(&mut self) {
        debug!("pan up!");
        self.disable_tailing();
        self.chart.pan_vertical(-0.1);
        self.repaint();
    }

    pub fn pan_down(&mut self) {
        debug!("pan down!");
        self.disable_tailing();
        self.chart.pan_vertical(0.1);
        self.repaint();
    }

    fn zoom_to_last(&mut self, tail_duration: f64) {
        self.time_estimator.predict();

        let end_time = self.time_estimator.get_estimate();
        let end = TimeStamp::new(end_time);
        let begin = end.clone() - tail_duration;
        let timespan = TimeSpan::new(begin, end);

        self.chart.fit_x_axis_to_timespan(&timespan);

        self.chart.fit_y_axis();
        self.repaint();
    }

    pub fn enable_tailing(&mut self, tail_duration: f64) {
        self.tailing = Some(tail_duration);
    }

    pub fn disable_tailing(&mut self) {
        self.tailing = None;
    }

    pub fn do_tailing(&mut self) {
        if let Some(x) = self.tailing {
            self.zoom_to_last(x);
        }
    }

    /// X axis has changed, either sync all axis, or redraw.
    /// Sync of all x-axis also triggers a redraw on own chart.
    fn handle_x_axis_change(&mut self) {
        // Repaint self:
        self.chart.fit_y_axis();
        self.repaint();

        // Adjust other axis:
        // The below call will skip this chart, since we are already
        // borrowed mutably here.
        self.app_state.borrow().sync_x_axis(&self);
    }

    /// Called from outside to synchronize the x-axis of this plot.
    pub fn sync_x_axis(&mut self, other_chart: &Self) {
        self.disable_tailing();
        self.chart.x_axis.copy_limits(&other_chart.chart.x_axis);
        self.chart.fit_y_axis();
        self.repaint();
    }

    /// Sync the cursor location from another chart.
    pub fn sync_cursor(&mut self, other_chart: &Self) {
        self.chart.cursor = other_chart.chart.cursor.clone();
        // .map(|(ts, _)| (ts, self.chart.y_axis.end()));
        self.repaint();
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.chart_layout.resize(width, height);
    }

    pub fn draw_on_canvas(&mut self, canvas: &cairo::Context) -> Inhibit {
        let size = get_size(&self.draw_area);

        // println!("Draw, width = {:?}, height= {:?}", width, height);
        canvas.set_font_size(12.0);
        let mut canvas2 = CairoCanvas::new(&canvas);

        let t1 = Instant::now();

        draw_chart(
            &self.chart,
            &mut canvas2,
            &mut self.chart_layout,
            &self.chart_options,
        );

        let t2 = Instant::now();
        let draw_duration = t2 - t1;
        // trace!("Drawing time: {:?}", draw_duration);

        // internal performance metric:
        let draw_seconds: f64 = draw_duration.as_secs_f64();
        self.perf_tracer
            .log_metric(&format!("META.{}.render_time", self.id), t1, draw_seconds);

        // Focus indicator!
        let is_focus = self.draw_area.is_focus();
        if is_focus {
            let padding = 1.0;
            gtk::render_focus(
                &self.draw_area.style_context(),
                &canvas,
                padding,
                padding,
                size.width - 2.0 * padding,
                size.height - 2.0 * padding,
            );
        }

        Inhibit(false)
    }
}

fn get_size(drawing_area: &gtk::DrawingArea) -> Size {
    let width = drawing_area.allocated_width() as f64;
    let height = drawing_area.allocated_height() as f64;
    Size::new(width, height)
}
