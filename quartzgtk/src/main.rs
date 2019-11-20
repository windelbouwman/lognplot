#[macro_use]
extern crate log;

use gio::prelude::*;
use gtk::prelude::*;
use std::time::Instant;

use gtk::Application;

use lognplot::chart::plot;
use lognplot::geometry::Size;
use lognplot::render::{CairoCanvas, Canvas};
use lognplot::server::run_server;

fn test1(canvas: &mut dyn Canvas, size: Size) {
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..900 {
        let f = (i as f64) * 0.1;
        x.push(f);
        y.push(20.0 * f.sin() + f * 2.0);
    }

    plot(canvas, x, y, size);
}

fn draw_on_canvas(drawingArea: &gtk::DrawingArea, canvas: &cairo::Context) -> Inhibit {
    let width = drawingArea.get_allocated_width();
    let height = drawingArea.get_allocated_height();
    let size = Size::new(width as f64, height as f64);
    // println!("Draw, width = {:?}, height= {:?}", width, height);
    let mut canvas2 = CairoCanvas::new(&canvas);

    let t1 = Instant::now();
    test1(&mut canvas2, size);
    let t2 = Instant::now();
    let draw_duration = t2 - t1;
    info!("Drawing time: {:?}", draw_duration);

    Inhibit(false)
}

fn on_key(draw_area: &gtk::DrawingArea, key: &gdk::EventKey) -> Inhibit {
    match key.get_keyval() {
        gdk::enums::key::Left => {
            println!("Left!");
        }
        gdk::enums::key::Right => {
            println!("Right!");
        }
        x => {
            println!("Key! {:?}", x);
        }
    };

    Inhibit(false)
}

fn main() {
    simple_logger::init().unwrap();
    info!("BOOTING QUARTZ TOOL");

    let application = Application::new(Some("com.github.windelbouwman.quartz"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        // First we get the file content.
        let glade_src = include_str!("gui.glade");
        // Then we call the Builder call.
        let builder = gtk::Builder::new_from_string(glade_src);

        // Connect draw event:
        let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
        draw_area.connect_draw(draw_on_canvas);

        // Connect application to window:
        let window: gtk::Window = builder.get_object("top_unit").unwrap();

        draw_area.connect_key_press_event(on_key);
        window.set_application(Some(app));
        window.show_all();
    });

    application.run(&[]);
}
