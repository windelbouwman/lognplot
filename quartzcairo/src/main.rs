#[macro_use]
extern crate log;

extern crate cairo;
extern crate gio;
extern crate gtk;

use quartzcanvas::{
    geometry::{Point, Size},
    style::Color,
    Canvas,
};
use quartzplot::plot;
use std::env::args;
use std::time::Instant;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::{Context, FontSlant, FontWeight};

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

struct CairoCanvas<'a> {
    cr: &'a cairo::Context,
}

impl<'a> CairoCanvas<'a> {
    fn new(cr: &'a cairo::Context) -> Self {
        Self { cr }
    }
}

impl<'a> Canvas for CairoCanvas<'a> {
    fn set_pen(&mut self, color: Color) {
        self.cr.set_source_rgb(
            color.r() as f64 / 255.0,
            color.g() as f64 / 255.0,
            color.b() as f64 / 255.0,
        );
    }

    fn print_text(&mut self, p: &Point, text: &str) {
        self.cr.move_to(p.x(), p.y());
        self.cr.show_text(text);
    }

    fn draw_line(&mut self, p1: &Point, p2: &Point) {
        self.cr.set_line_width(3.0);
        self.cr.move_to(p1.x(), p1.y());
        self.cr.line_to(p2.x(), p2.y());
        self.cr.stroke();
    }
}

fn build_ui(application: &gtk::Application) {
    drawable(application, 500, 500, |area, cr| {
        let width = area.get_allocated_width();
        let height = area.get_allocated_height();
        let size = Size::new(width as f64, height as f64);
        let t1 = Instant::now();
        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);

        cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
        cr.paint();

        let mut cc = CairoCanvas::new(cr);
        test1(&mut cc, size);
        let t2 = Instant::now();
        let draw_duration = t2 - t1;
        info!("Drawing time: {:?}", draw_duration);

        Inhibit(false)
    });
}

fn main() {
    simple_logger::init().unwrap();
    info!("BOOTING QUARTZ TOOL");

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.cairotest"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

pub fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where
    F: Fn(&DrawingArea, &Context) -> Inhibit + 'static,
{
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);

    window.add(&drawing_area);
    window.show_all();
}
