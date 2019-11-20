use gio::prelude::*;
use gtk::prelude::*;

use gtk::Application;

use quartzcanvas::{
    geometry::{Point, Size},
    style::Color,
    Canvas,
};
use quartzplot::plot;

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

fn draw_on_canvas(drawingArea: &gtk::DrawingArea, canvas: &cairo::Context) -> Inhibit {
    let width = drawingArea.get_allocated_width();
    let height = drawingArea.get_allocated_height();
    let size = Size::new(width as f64, height as f64);
    // println!("Draw, width = {:?}, height= {:?}", width, height);
    let mut canvas2 = CairoCanvas::new(&canvas);
    test1(&mut canvas2, size);

    Inhibit(false)
}

fn on_key(draw_area: &gtk::DrawingArea, key: &gdk::EventKey) -> Inhibit {
    match key.get_keyval() {
        gdk::enums::key::Left => {
            println!("Left!");
        },
        gdk::enums::key::Right => {
            println!("Right!");
        },
        x => {
            println!("Key! {:?}", x);
        }
    };

    Inhibit(false)
}

fn main() {
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
