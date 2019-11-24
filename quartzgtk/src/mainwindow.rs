use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use lognplot::chart::{Chart, Curve, CurveData};
use lognplot::geometry::Size;
use lognplot::render::{draw_chart, CairoCanvas};
use lognplot::time::TimeStamp;
use lognplot::tsdb::{Observation, Sample, TsDbHandle};

/// Struct with some GUI state in it which will be shown in the GUI.
struct GuiState {
    chart: Chart,
    // db: TsDbHandle,
}

fn new_chart(db: TsDbHandle) -> Chart {
    db.new_trace("Trace0");

    // Create data:
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..900 {
        let f = (i as f64) * 0.1;
        let v = 20.0 * f.sin() + f * 2.0;

        // Construct raw data:
        x.push(f);
        y.push(v);

        // Add observations:
        let timestamp = TimeStamp::new(f);
        let sample = Sample::new(v + 20.0);
        let observation = Observation::new(timestamp, sample);
        // db.add_value("bla", observation);
    }

    info!("Plotting len(x)= {:?} len(y)= {:?}", x.len(), y.len());
    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    let curve_data = CurveData::points(x, y);
    let curve = Curve::new(curve_data);
    chart.add_curve(curve);

    let tsdb_data = CurveData::trace("Trace0", db);
    let curve2 = Curve::new(tsdb_data);

    chart.add_curve(curve2);
    chart.autoscale();
    chart
}

impl GuiState {
    fn new(db: TsDbHandle) -> Self {
        GuiState {
            chart: new_chart(db.clone()),
            // db,
        }
    }

    fn into_handle(self) -> GuiStateHandle {
        Rc::new(RefCell::new(self))
    }
}

type GuiStateHandle = Rc<RefCell<GuiState>>;

pub fn open_gui(db_handle: TsDbHandle) {
    let app_state = GuiState::new(db_handle.clone()).into_handle();

    let application = Application::new(Some("com.github.windelbouwman.quartz"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(move |app| build_ui(app, app_state.clone()));

    application.run(&[]);
}

fn build_ui(app: &gtk::Application, app_state: GuiStateHandle) {
    // First we get the file content.
    let glade_src = include_str!("gui.glade");
    // Then we call the Builder call.
    let builder = gtk::Builder::new_from_string(glade_src);

    // Connect draw event:
    let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
    let draw_app_handle = app_state.clone();
    draw_area.connect_draw(move |a, c| draw_on_canvas(a, c, draw_app_handle.clone()));

    // Connect key event:
    let key_handler_app_state = app_state.clone();
    draw_area.connect_key_press_event(move |a, k| on_key(a, k, key_handler_app_state.clone()));

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    window.set_application(Some(app));
    window.show_all();
}

fn draw_on_canvas<'t>(
    drawing_area: &gtk::DrawingArea,
    canvas: &cairo::Context,
    app_state: GuiStateHandle,
) -> Inhibit {
    let width = drawing_area.get_allocated_width();
    let height = drawing_area.get_allocated_height();
    let size = Size::new(width as f64, height as f64);
    // println!("Draw, width = {:?}, height= {:?}", width, height);
    let mut canvas2 = CairoCanvas::new(&canvas);

    let t1 = Instant::now();

    draw_chart(&app_state.borrow().chart, &mut canvas2, size);

    let t2 = Instant::now();
    let draw_duration = t2 - t1;
    info!("Drawing time: {:?}", draw_duration);

    Inhibit(false)
}

fn on_key(draw_area: &gtk::DrawingArea, key: &gdk::EventKey, app_state: GuiStateHandle) -> Inhibit {
    match key.get_keyval() {
        gdk::enums::key::Up => {
            app_state.borrow_mut().chart.pan_vertical(-0.1);
            println!("Up!");
        }
        gdk::enums::key::Left => {
            app_state.borrow_mut().chart.pan_horizontal(-0.1);
            println!("Left!");
        }
        gdk::enums::key::Right => {
            app_state.borrow_mut().chart.pan_horizontal(0.1);
            println!("Right!");
        }
        gdk::enums::key::KP_Add => {
            app_state.borrow_mut().chart.zoom_horizontal(0.1);
            println!("Plus!");
        }
        gdk::enums::key::KP_Subtract => {
            app_state.borrow_mut().chart.zoom_horizontal(-0.1);
            println!("Minus!");
        }
        gdk::enums::key::Return => {
            app_state.borrow_mut().chart.autoscale();
            println!("Enter bar!");
        }

        x => {
            println!("Key! {:?}", x);
        }
    };
    draw_area.queue_draw();

    Inhibit(false)
}
