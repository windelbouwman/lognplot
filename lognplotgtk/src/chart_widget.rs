//! Deals with drawing on the chart drawing area, as well as keyboard handling.
//!
//! Also implement plot control splitting, and adding of extra buttons for split, clear plot etc.

use gdk::Key;
use gtk::prelude::*;

use super::chart_state::{ChartState, ChartStateHandle};
use crate::state::GuiStateHandle;

/// Create new chart area with extra buttons around it
/// to enable splitting in vertical and horizontal direction
pub fn create_new_chart_area(app_state: &GuiStateHandle) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let draw_area = gtk::DrawingArea::new();
    vbox.append(&draw_area);
    massage_drawing_area(&draw_area);

    // generate new unique chart id based on amount of charts so far:
    let chart_id = format!("chart{}", app_state.borrow().num_charts() + 1);

    let chart_state1 = setup_drawing_area(draw_area, app_state.clone(), &chart_id);

    let box2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let button_clear_plot = gtk::Button::new();
    button_clear_plot.set_label("Clear plot");
    box2.append(&button_clear_plot);

    vbox.append(&box2);

    button_clear_plot.connect_clicked(clone!(
        #[strong]
        chart_state1,
        move |_| {
            chart_state1.borrow_mut().clear_curves();
        }
    ));

    app_state.borrow_mut().add_chart(chart_state1.clone());
    vbox
}

/// Apply various settings to the drawing area.
fn massage_drawing_area(new_chart_area: &gtk::DrawingArea) {
    new_chart_area.set_hexpand(true);
    new_chart_area.set_vexpand(true);
    new_chart_area.set_size_request(200, 200);
    new_chart_area.set_can_focus(true);
    new_chart_area.set_focusable(true);
    new_chart_area.set_sensitive(true);
    new_chart_area.set_receives_default(true);
}

fn setup_drawing_area(
    draw_area: gtk::DrawingArea,
    app_state: GuiStateHandle,
    chart_id: &str,
) -> ChartStateHandle {
    let db = { app_state.borrow().db.clone() };
    let perf_tracer = app_state.borrow().get_perf_tracer();

    let chart_state =
        ChartState::new(db, perf_tracer, app_state, draw_area.clone(), chart_id).into_handle();

    // Connect draw event:
    draw_area.set_draw_func(clone!(
        #[strong]
        chart_state,
        move |_, c, width, height| chart_state.borrow_mut().draw_on_canvas(
            c,
            width as f64,
            height as f64
        )
    ));

    // Connect drop event:
    let formats = gdk::ContentFormats::builder()
        .add_type(glib::Type::STRING)
        .add_mime_type(super::mime_types::SIGNAL_NAMES_MIME_TYPE)
        .build();
    let drop_target = gtk::DropTarget::builder()
        .actions(gdk::DragAction::COPY)
        .formats(&formats)
        .build();
    drop_target.connect_drop(clone!(
        #[strong]
        chart_state,
        move |_target, value, _x, _y| {
            info!("Drop");
            let mime_payload: String = value.get::<String>().expect("Must work!!");
            if let Ok(signal_names) = serde_json::from_str::<Vec<String>>(&mime_payload) {
                info!("DROP {:?}", signal_names);
                for signal_name in signal_names {
                    chart_state.borrow_mut().add_curve(&signal_name);
                }
                true
            } else {
                error!(
                    "Error in drop action, could not plot mime data: {}",
                    mime_payload
                );
                false
            }
        }
    ));
    draw_area.add_controller(drop_target);

    let gesture_click = gtk::GestureClick::new();
    gesture_click.connect_pressed(clone!(
        #[strong]
        draw_area,
        move |_gesture, _n, _x, _y| {
            info!("Grab focus on chart");
            if draw_area.grab_focus() {
                debug!("Yes focus");
            } else {
                error!("No focus");
            }
        }
    ));
    draw_area.add_controller(gesture_click);

    let gesture_pan = gtk::GesturePan::new(gtk::Orientation::Horizontal);
    gesture_pan.connect_drag_begin(clone!(
        #[strong]
        chart_state,
        move |_controller, x, y| {
            debug!("Gesture begin press! {},{}", x, y);
            chart_state.borrow_mut().start_drag(x, y);
        }
    ));
    gesture_pan.connect_drag_update(clone!(
        #[strong]
        chart_state,
        move |_controller, x, y| {
            debug!("Gesture update press! {},{}", x, y);
            chart_state.borrow_mut().move_drag(x, y);
        }
    ));
    draw_area.add_controller(gesture_pan);

    let motion_controller = gtk::EventControllerMotion::new();
    motion_controller.connect_leave(clone!(
        #[strong]
        chart_state,
        move |_| {
            debug!("Mouse leave!");
            chart_state.borrow_mut().set_cursor(None);
        }
    ));

    motion_controller.connect_motion(clone!(
        #[strong]
        chart_state,
        move |_, x, y| {
            let mut chart = chart_state.borrow_mut();
            // debug!("Mouse motion! {},{}", x, y);
            chart.set_cursor(Some((x, y)));
        }
    ));
    draw_area.add_controller(motion_controller);

    let scroll_controller =
        gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);

    scroll_controller.connect_scroll(clone!(
        #[strong]
        chart_state,
        move |_, dx, dy| {
            let mut chart = chart_state.borrow_mut();
            debug!("Scroll wheel event! dx={:?}, dy={:?}", dx, dy);
            if dy < 0.0 {
                chart.zoom_in_horizontal();
            } else if dy > 0.0 {
                chart.zoom_out_horizontal();
            }

            glib::signal::Propagation::Stop
        }
    ));
    draw_area.add_controller(scroll_controller);

    draw_area.connect_resize(clone!(
        #[strong]
        chart_state,
        move |_draw_area, width, height| {
            chart_state.borrow_mut().resize(width as f64, height as f64);
        }
    ));

    // Connect key event:
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(clone!(
        #[strong]
        chart_state,
        move |_, key, _, _| {
            let mut chart = chart_state.borrow_mut();

            chart.disable_tailing();
            match key {
                Key::Up | Key::w => {
                    chart.pan_up();
                }
                Key::Down | Key::s => {
                    chart.pan_down();
                }
                Key::Left | Key::a => {
                    chart.pan_left();
                }
                Key::Right | Key::d => {
                    chart.pan_right();
                }
                Key::i => {
                    chart.zoom_in_vertical();
                }
                Key::k => {
                    chart.zoom_out_vertical();
                }
                Key::KP_Add | Key::l => {
                    chart.zoom_in_horizontal();
                }
                Key::KP_Subtract | Key::j => {
                    chart.zoom_out_horizontal();
                }
                Key::Home | Key::Return => {
                    chart.zoom_fit();
                }
                Key::BackSpace => {
                    chart.clear_curves();
                }
                Key::_1 => {
                    chart.set_cursor1();
                }
                Key::_2 => {
                    chart.set_cursor2();
                }
                other_key => {
                    println!("Key! {:?}", other_key);
                }
            };

            glib::signal::Propagation::Stop
        }
    ));
    draw_area.add_controller(key_controller);

    chart_state
}
