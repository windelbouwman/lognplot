// Deals with drawing on the chart drawing area, as well as keyboard handling.

use gtk::prelude::*;

use std::time::Instant;

use super::GuiStateHandle;
use lognplot::geometry::Size;
use lognplot::render::{draw_chart, CairoCanvas};

pub fn setup_drawing_area(draw_area: gtk::DrawingArea, app_state: GuiStateHandle) {
    // Connect draw event:
    let draw_app_handle = app_state.clone();
    draw_area.connect_draw(move |a, c| draw_on_canvas(a, c, draw_app_handle.clone()));

    // Connect drop event:
    let targets = vec![gtk::TargetEntry::new(
        "text/plain",
        gtk::TargetFlags::empty(),
        0,
    )];
    draw_area.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    let app_state_drag_data_received = app_state.clone();
    draw_area.connect_drag_data_received(move |w, _dc, _x, _y, data, _info, _time| {
        let signal_name: String = data.get_text().expect("Must work!!").to_string();
        println!("DROP {}", signal_name);
        app_state_drag_data_received
            .borrow_mut()
            .add_curve(&signal_name);
        w.queue_draw();
    });

    // Connect key event:
    let key_handler_app_state = app_state.clone();
    draw_area.connect_key_press_event(move |a, k| on_key(a, k, key_handler_app_state.clone()));

    // Click event:
    // gtk::widget_set_focus_on_click();
    // draw_area.set_focus_on_click(true);
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
    canvas.set_font_size(14.0);
    let mut canvas2 = CairoCanvas::new(&canvas);

    let t1 = Instant::now();

    draw_chart(&app_state.borrow().chart, &mut canvas2, size);

    let t2 = Instant::now();
    let draw_duration = t2 - t1;
    info!("Drawing time: {:?}", draw_duration);

    // Focus indicator!
    let is_focus = drawing_area.is_focus();
    if is_focus {
        canvas.move_to(10.0, 10.0);
        canvas.show_text("KEY FOCUS");
    }

    Inhibit(false)
}

fn on_key(draw_area: &gtk::DrawingArea, key: &gdk::EventKey, app_state: GuiStateHandle) -> Inhibit {
    app_state.borrow_mut().disable_tailing();
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
            app_state.borrow_mut().zoom_fit();
        }
        gdk::enums::key::BackSpace => {
            info!("Kill all signals!");
            app_state.borrow_mut().clear_curves();
        }

        x => {
            println!("Key! {:?}", x);
        }
    };
    draw_area.queue_draw();

    Inhibit(false)
}
