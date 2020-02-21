// Deals with drawing on the chart drawing area, as well as keyboard handling.

use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::session::DashBoardItem;
use lognplot::chart::{Chart, Curve, CurveData};
use lognplot::geometry::Size;
use lognplot::render::{draw_chart, CairoCanvas};
use lognplot::render::{x_pixel_to_domain, x_pixels_to_domain};
use lognplot::time::TimeStamp;
use lognplot::tsdb::DataChangeEvent;
use lognplot::tsdb::TsDbHandle;

pub struct ChartState {
    pub chart: Chart,
    pub db: TsDbHandle,
    color_wheel: Vec<String>,
    color_index: usize,
    tailing: Option<f64>,

    drag: Option<(f64, f64)>,
    draw_area: gtk::DrawingArea,
}

fn new_chart() -> Chart {
    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    chart
}

impl ChartState {
    pub fn new(db: TsDbHandle, draw_area: gtk::DrawingArea) -> Self {
        let chart = new_chart();
        let color_wheel = vec!["blue".to_string(), "red".to_string(), "green".to_string()];

        ChartState {
            chart,
            db,
            color_wheel,
            color_index: 0,
            tailing: None,
            drag: None,
            draw_area,
        }
    }

    pub fn into_handle(self) -> ChartStateHandle {
        Rc::new(RefCell::new(self))
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

    pub fn next_color(&mut self) -> String {
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
    pub fn handle_event(&self, event: &DataChangeEvent) {
        // Check if we must update the chart:
        let update = event
            .changed_signals
            .iter()
            .any(|n| self.chart.has_signal(n));
        if update {
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
    pub fn move_drag(&mut self, size: Size, x: f64, y: f64) {
        self.disable_tailing();
        if let Some((prev_x, prev_y)) = self.drag {
            let dx = x - prev_x;
            let dy = y - prev_y;
            self.do_drag(size, dx, dy);
        }
        self.drag = Some((x, y));
    }

    /// Drag the plot by the given amount.
    fn do_drag(&mut self, size: Size, dx: f64, dy: f64) {
        debug!("Drag! {}, {} ", dx, dy);

        let amount = x_pixels_to_domain(size, &self.chart.x_axis, dx);

        self.chart.pan_horizontal_absolute(-amount);
        // TODO: pan vertical as well?
        // TODO: idea: auto fit vertically?
        self.chart.fit_y_axis();
        // self.chart.pan_vertical(dy* 0.001);
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

    pub fn zoom_in_horizontal(&mut self, around: Option<(f64, Size)>) {
        debug!("Zoom in horizontal");
        self.zoom_horizontal(-0.1, around);
    }

    pub fn zoom_out_horizontal(&mut self, around: Option<(f64, Size)>) {
        debug!("Zoom out horizontal");
        self.zoom_horizontal(0.1, around);
    }

    fn zoom_horizontal(&mut self, amount: f64, around: Option<(f64, Size)>) {
        let around = around.map(|p| {
            let (pixel, size) = p;
            x_pixel_to_domain(pixel, &self.chart.x_axis, size)
        });
        self.disable_tailing();
        self.chart.zoom_horizontal(amount, around);
        self.chart.fit_y_axis();
        self.repaint();
    }

    pub fn set_cursor(&mut self, loc: Option<(f64, Size)>) {
        if let Some((pixel, size)) = loc {
            let timestamp = x_pixel_to_domain(pixel, &self.chart.x_axis, size);
            let timestamp = TimeStamp::new(timestamp);
            self.chart.cursor = Some(timestamp);
        } else {
            self.chart.cursor = None;
        }
        self.repaint();
    }

    pub fn pan_left(&mut self) {
        debug!("pan left!");
        self.disable_tailing();
        self.chart.pan_horizontal_relative(-0.1);
        self.chart.fit_y_axis();
        self.repaint();
    }

    pub fn pan_right(&mut self) {
        debug!("Pan right!");
        self.disable_tailing();
        self.chart.pan_horizontal_relative(0.1);
        self.chart.fit_y_axis();
        self.repaint();
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

    pub fn zoom_to_last(&mut self, tail_duration: f64) {
        self.chart.zoom_to_last(tail_duration);
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
}

pub type ChartStateHandle = Rc<RefCell<ChartState>>;

pub fn setup_drawing_area(draw_area: gtk::DrawingArea, db: TsDbHandle) -> ChartStateHandle {
    // Always get mouse pointer motion:
    draw_area.add_events(gdk::EventMask::ENTER_NOTIFY_MASK);
    draw_area.add_events(gdk::EventMask::POINTER_MOTION_MASK);
    draw_area.add_events(gdk::EventMask::LEAVE_NOTIFY_MASK);

    let chart_state = ChartState::new(db, draw_area.clone()).into_handle();

    // Connect draw event:
    draw_area.connect_draw(
        clone!(@strong chart_state => move |a, c| { draw_on_canvas(a, c, chart_state.clone()) } ),
    );

    // Connect drop event:
    let targets = vec![gtk::TargetEntry::new(
        super::mime_types::SIGNAL_NAMES_MIME_TYPE,
        gtk::TargetFlags::empty(),
        0,
    )];
    draw_area.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

    draw_area.connect_drag_data_received(
        clone!(@strong chart_state => move |w, _dc, _x, _y, data, _info, _time| {
            let mime_payload: String = data.get_text().expect("Must work!!").to_string();
            if let Ok(signal_names) = serde_json::from_str::<Vec<String>>(&mime_payload) {
                info!("DROP {:?}", signal_names);
                for signal_name in signal_names {
                    chart_state
                    .borrow_mut()
                    .add_curve(&signal_name);
                }
                w.grab_focus();
            } else {
                error!("Error in drop action, could not plot mime data: {}", mime_payload);
            }
        }),
    );

    draw_area.connect_button_press_event(clone!(@strong chart_state => move |w, e| {
        let pos = e.get_position();
        debug!("Mouse press! {:?}", pos);
        chart_state.borrow_mut().start_drag(pos.0, pos.1);
        w.grab_focus();
        Inhibit(false)
    }));

    draw_area.connect_leave_notify_event(clone!(@strong chart_state => move |_w, _e| {
        debug!("Mouse leave!");
        chart_state.borrow_mut().set_cursor(None);
        Inhibit(false)
    }));

    draw_area.connect_motion_notify_event(clone!(@strong chart_state => move |w, e| {
        on_motion_event(w, e, chart_state.clone())
    }));

    draw_area.connect_scroll_event(clone!(@strong chart_state => move |w, e| {
        on_scroll_event(w, e, chart_state.clone())
    }));

    // Connect key event:
    draw_area.connect_key_press_event(
        clone!(@strong chart_state => move |_a, k| { on_key(k, chart_state.clone()) } ),
    );

    setup_tailing_timer(chart_state.clone());

    chart_state
}

fn on_motion_event(
    drawing_area: &gtk::DrawingArea,
    e: &gdk::EventMotion,
    chart_state: ChartStateHandle,
) -> Inhibit {
    let pos = e.get_position();
    debug!("Mouse motion! {:?}", pos);
    let size = get_size(drawing_area);

    chart_state
        .borrow_mut()
        .set_cursor(Some((pos.0, size.clone())));

    if e.get_state().contains(gdk::ModifierType::BUTTON1_MASK) {
        chart_state.borrow_mut().move_drag(size, pos.0, pos.1);
    }
    drawing_area.queue_draw();

    Inhibit(false)
}

fn on_scroll_event(
    drawing_area: &gtk::DrawingArea,
    e: &gdk::EventScroll,
    chart_state: ChartStateHandle,
) -> Inhibit {
    debug!(
        "Scroll wheel event! {:?}, {:?}, {:?}",
        e,
        e.get_delta(),
        e.get_direction()
    );
    let size = get_size(drawing_area);
    let pixel_x_pos = e.get_position().0;
    let around = Some((pixel_x_pos, size));
    match e.get_direction() {
        gdk::ScrollDirection::Up => {
            chart_state.borrow_mut().zoom_in_horizontal(around);
        }
        gdk::ScrollDirection::Down => {
            chart_state.borrow_mut().zoom_out_horizontal(around);
        }
        gdk::ScrollDirection::Left => {
            chart_state.borrow_mut().pan_left();
        }
        gdk::ScrollDirection::Right => {
            chart_state.borrow_mut().pan_right();
        }
        _ => {}
    }
    Inhibit(false)
}

fn get_size(drawing_area: &gtk::DrawingArea) -> Size {
    let width = drawing_area.get_allocated_width() as f64;
    let height = drawing_area.get_allocated_height() as f64;
    Size::new(width, height)
}

fn draw_on_canvas(
    drawing_area: &gtk::DrawingArea,
    canvas: &cairo::Context,
    chart_state: ChartStateHandle,
) -> Inhibit {
    let size = get_size(drawing_area);

    // println!("Draw, width = {:?}, height= {:?}", width, height);
    canvas.set_font_size(14.0);
    let mut canvas2 = CairoCanvas::new(&canvas);

    let t1 = Instant::now();

    draw_chart(&chart_state.borrow().chart, &mut canvas2, size.clone());

    let t2 = Instant::now();
    let draw_duration = t2 - t1;
    trace!("Drawing time: {:?}", draw_duration);

    // TODO: re-enable this internal performance metric:
    // let draw_seconds: f64 = draw_duration.as_secs_f64();
    // app_state
    //     .borrow()
    //     .log_meta_metric("META_chart_render_time", t1, draw_seconds);

    // Focus indicator!
    let is_focus = drawing_area.is_focus();
    if is_focus {
        let padding = 1.0;
        gtk::render_focus(
            &drawing_area.get_style_context(),
            &canvas,
            padding,
            padding,
            size.width - 2.0 * padding,
            size.height - 2.0 * padding,
        );
    }

    Inhibit(false)
}

/// Setup a timer to implement tailing of signals.
fn setup_tailing_timer(chart_state: ChartStateHandle) {
    // Refreshing timer!
    let tick = move || {
        chart_state.borrow_mut().do_tailing();
        gtk::prelude::Continue(true)
    };
    gtk::timeout_add(100, tick);
}

fn on_key(key: &gdk::EventKey, chart_state: ChartStateHandle) -> Inhibit {
    chart_state.borrow_mut().disable_tailing();
    match key.get_keyval() {
        gdk::enums::key::Up | gdk::enums::key::w => {
            chart_state.borrow_mut().pan_up();
        }
        gdk::enums::key::Down | gdk::enums::key::s => {
            chart_state.borrow_mut().pan_down();
        }
        gdk::enums::key::Left | gdk::enums::key::a => {
            chart_state.borrow_mut().pan_left();
        }
        gdk::enums::key::Right | gdk::enums::key::d => {
            chart_state.borrow_mut().pan_right();
        }
        gdk::enums::key::i => {
            chart_state.borrow_mut().zoom_in_vertical();
        }
        gdk::enums::key::k => {
            chart_state.borrow_mut().zoom_out_vertical();
        }
        gdk::enums::key::KP_Add | gdk::enums::key::l => {
            chart_state.borrow_mut().zoom_in_horizontal(None);
        }
        gdk::enums::key::KP_Subtract | gdk::enums::key::j => {
            chart_state.borrow_mut().zoom_out_horizontal(None);
        }
        gdk::enums::key::Home | gdk::enums::key::Return => {
            chart_state.borrow_mut().zoom_fit();
        }
        gdk::enums::key::BackSpace => {
            chart_state.borrow_mut().clear_curves();
        }

        x => {
            println!("Key! {:?}", x);
        }
    };

    Inhibit(true)
}
