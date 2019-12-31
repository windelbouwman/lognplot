// Deals with drawing on the chart drawing area, as well as keyboard handling.

use gtk::prelude::*;

use std::time::Instant;

use super::GuiStateHandle;
use lognplot::geometry::Size;
use lognplot::render::{draw_chart, CairoCanvas};

pub fn setup_drawing_area(draw_area: gtk::DrawingArea, app_state: GuiStateHandle) {
    // Connect draw event:
    draw_area.connect_draw(
        clone!(@strong app_state => move |a, c| { draw_on_canvas(a, c, app_state.clone()) } ),
    );

    // Connect drop event:
    let targets = vec![gtk::TargetEntry::new(
        super::mime_types::SIGNAL_NAMES_MIME_TYPE,
        gtk::TargetFlags::empty(),
        0,
    )];
    draw_area.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

    draw_area.connect_drag_data_received(
        clone!(@strong app_state => move |w, _dc, _x, _y, data, _info, _time| {
            let signal_names: String = data.get_text().expect("Must work!!").to_string();
            info!("DROP {}", signal_names);
            for signal_name in signal_names.split(":") {
                app_state
                .borrow_mut()
                .add_curve(&signal_name);
            }
            w.queue_draw();
            w.grab_focus();
        }),
    );

    draw_area.connect_button_press_event(clone!(@strong app_state => move |w, e| {
        let pos = e.get_position();
        debug!("Mouse press! {:?}", pos);
        app_state.borrow_mut().start_drag(pos.0, pos.1);

        w.grab_focus();
        Inhibit(false)
    }));

    draw_area.connect_motion_notify_event(clone!(@strong app_state => move |w, e| {
        let pos = e.get_position();
        debug!("Mouse motion! {:?}", pos);
        let size = get_size(w);
        app_state
            .borrow_mut()
            .move_drag(size, pos.0, pos.1);

        // if e.get_state(). & gdk::BUTTON1_mask {
        w.queue_draw();
        // }

        Inhibit(false)
    }));

    draw_area.connect_scroll_event(clone!(@strong app_state => move |w, e| {
        // println!("Scroll wheel event! {:?}, {:?}, {:?}", e, e.get_delta(), e.get_direction());
        match e.get_direction() {
            gdk::ScrollDirection::Up => {
                app_state.borrow_mut().zoom_in_horizontal();
            },
            gdk::ScrollDirection::Down => {
                app_state.borrow_mut().zoom_out_horizontal();
            },
            _ => {}
        }
        w.queue_draw();
        Inhibit(false)
    }));

    // Connect key event:
    draw_area.connect_key_press_event(
        clone!(@strong app_state => move |a, k| { on_key(a, k, app_state.clone()) } ),
    );

    // Click event:
    // gtk::widget_set_focus_on_click();
    // draw_area.set_focus_on_click(true);
}

fn get_size(drawing_area: &gtk::DrawingArea) -> Size {
    let width = drawing_area.get_allocated_width() as f64;
    let height = drawing_area.get_allocated_height() as f64;
    let size = Size::new(width, height);
    size
}

fn draw_on_canvas(
    drawing_area: &gtk::DrawingArea,
    canvas: &cairo::Context,
    app_state: GuiStateHandle,
) -> Inhibit {
    let size = get_size(drawing_area);

    // println!("Draw, width = {:?}, height= {:?}", width, height);
    canvas.set_font_size(14.0);
    let mut canvas2 = CairoCanvas::new(&canvas);

    let t1 = Instant::now();

    draw_chart(&app_state.borrow().chart, &mut canvas2, size.clone());

    let t2 = Instant::now();
    let draw_duration = t2 - t1;
    trace!("Drawing time: {:?}", draw_duration);
    let draw_seconds: f64 = draw_duration.as_secs_f64();

    app_state
        .borrow()
        .log_meta_metric("META_chart_render_time", t1, draw_seconds);

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
            app_state.borrow_mut().zoom_in_vertical();
        }
        gdk::enums::key::k => {
            app_state.borrow_mut().zoom_out_vertical();
        }
        gdk::enums::key::KP_Add | gdk::enums::key::l => {
            app_state.borrow_mut().zoom_in_horizontal();
        }
        gdk::enums::key::KP_Subtract | gdk::enums::key::j => {
            app_state.borrow_mut().zoom_out_horizontal();
        }
        gdk::enums::key::Home | gdk::enums::key::Return => {
            app_state.borrow_mut().zoom_fit();
        }
        gdk::enums::key::BackSpace => {
            app_state.borrow_mut().clear_curves();
        }

        x => {
            println!("Key! {:?}", x);
        }
    };
    draw_area.queue_draw();

    Inhibit(false)
}
