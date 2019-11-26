use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use std::time::Instant;

use lognplot::geometry::Size;
use lognplot::render::{draw_chart, CairoCanvas};
use lognplot::tsdb::TsDbHandle;

use super::{GuiState, GuiStateHandle};

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
        gdk::enums::key::Up | gdk::enums::key::w => {
            app_state.borrow_mut().pan_up();
        }
        gdk::enums::key::s => {
            app_state.borrow_mut().pan_down();
        }
        gdk::enums::key::Left | gdk::enums::key::a => {
            app_state.borrow_mut().pan_left();
        }
        gdk::enums::key::Right | gdk::enums::key::d => {
            app_state.borrow_mut().pan_right();
        }
        gdk::enums::key::i => {
            info!("Zoom vertical");
            app_state.borrow_mut().chart.zoom_vertical(0.1);
        }
        gdk::enums::key::k => {
            info!("Zoom vertical");
            app_state.borrow_mut().chart.zoom_vertical(-0.1);
        }
        gdk::enums::key::KP_Add | gdk::enums::key::l => {
            app_state.borrow_mut().zoom_in_horizontal();
        }
        gdk::enums::key::KP_Subtract | gdk::enums::key::j => {
            app_state.borrow_mut().zoom_out_horizontal();
        }
        gdk::enums::key::Return => {
            info!("Autoscale!");
            app_state.borrow_mut().chart.autoscale();
        }

        x => {
            println!("Key! {:?}", x);
        }
    };
    draw_area.queue_draw();

    Inhibit(false)
}
