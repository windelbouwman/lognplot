#[macro_use]
extern crate log;

extern crate cairo;
extern crate gio;
extern crate gtk;

use quartzcanvas::{geometry::Point, style::Color, Canvas};
use quartzplot::plot;
use std::env::args;
use std::f64::consts::PI;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;

use cairo::{Context, FontSlant, FontWeight};

fn test1(canvas: &mut dyn Canvas) {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 8.0];
    let y = vec![9.0, 2.2, 5.5, 2.2, 1.2, 1.7];

    plot(canvas, x, y);
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
    drawable(application, 500, 500, |_, cr| {
        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);

        // cr.scale(500f64, 500f64);

        cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
        cr.paint();

        cr.set_line_width(3.0);

        // border
        cr.set_source_rgb(0.3, 0.3, 0.3);
        cr.rectangle(0.0, 0.0, 500.0, 500.0);
        cr.stroke();
        cr.set_line_width(0.03);

        let mut cc = CairoCanvas::new(cr);
        test1(&mut cc);

        /*
                // draw circle
                cr.arc(0.5, 0.5, 0.4, 0.0, PI * 2.);
                cr.stroke();

                // mouth
                let mouth_top = 0.68;
                let mouth_width = 0.38;

                let mouth_dx = 0.10;
                let mouth_dy = 0.10;

                cr.move_to(0.50 - mouth_width / 2.0, mouth_top);
                cr.curve_to(
                    0.50 - mouth_dx,
                    mouth_top + mouth_dy,
                    0.50 + mouth_dx,
                    mouth_top + mouth_dy,
                    0.50 + mouth_width / 2.0,
                    mouth_top,
                );

                println!("Extents: {:?}", cr.fill_extents());

                cr.stroke();

                let eye_y = 0.38;
                let eye_dx = 0.15;
                cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI * 2.);
                cr.fill();

                cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI * 2.);
                cr.fill();
        */
        Inhibit(false)
    });
    /*
    drawable(application, 500, 500, |_, cr| {
        cr.scale(500f64, 500f64);

        cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
        cr.set_font_size(0.35);

        cr.move_to(0.04, 0.53);
        cr.show_text("Hello");

        cr.move_to(0.27, 0.65);
        cr.text_path("void");
        cr.set_source_rgb(0.5, 0.5, 1.0);
        cr.fill_preserve();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.set_line_width(0.01);
        cr.stroke();

        cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        cr.arc(0.04, 0.53, 0.02, 0.0, PI * 2.);
        cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);
        cr.fill();

        Inhibit(false)
    });
    */
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
