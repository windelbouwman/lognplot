//! Deals with drawing on the chart drawing area, as well as keyboard handling.
//!
//! Also implement plot control splitting, and adding of extra buttons for split, clear plot etc.

use gtk::prelude::*;

use super::chart_state::{ChartState, ChartStateHandle};
use crate::state::GuiStateHandle;

/// Create new chart area with extra buttons around it
/// to enable splitting in vertical and horizontal direction
pub fn create_new_chart_area(app_state: &GuiStateHandle, parent_box: gtk::Box) {
    let box1 = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let draw_area: gtk::DrawingArea = gtk::DrawingArea::new();
    massage_drawing_area(&draw_area);
    box1.pack_start(&draw_area, true, true, 0);

    // generate new unique chart id based on amount of charts so far:
    let chart_id = format!("chart{}", app_state.borrow().num_charts() + 1);

    let chart_state1 = setup_drawing_area(draw_area, app_state.clone(), &chart_id);

    // Create split buttons:
    let box2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let button_split_vertical = gtk::Button::new();
    button_split_vertical.set_label("Split vertical");
    box2.pack_start(&button_split_vertical, false, false, 0);

    let button_split_horizontal = gtk::Button::new();
    button_split_horizontal.set_label("Split horizontal");
    box2.pack_start(&button_split_horizontal, false, false, 0);

    let button_close = gtk::Button::new();
    button_close.set_label("Close");
    box2.pack_start(&button_close, false, false, 0);

    let button_clear_plot = gtk::Button::new();
    button_clear_plot.set_label("Clear plot");
    box2.pack_start(&button_clear_plot, false, false, 0);

    box1.pack_start(&box2, false, false, 0);
    box1.show_all();

    button_split_vertical.connect_clicked(clone!(@strong app_state, @strong box1 => move |_| {
        info!("Split vertically");
        split_chart(&app_state, &box1, gtk::Orientation::Vertical);
    }));

    button_split_horizontal.connect_clicked(clone!(@strong app_state, @strong box1 => move |_| {
        info!("Split horizontally");
        split_chart(&app_state, &box1, gtk::Orientation::Horizontal);
    }));

    button_close.connect_clicked(clone!(@strong box1 => move |_| {
        info!("Close chart!");
        close_chart(&box1);
    }));

    button_clear_plot.connect_clicked(clone!(@strong chart_state1 => move |_| {
        chart_state1.borrow_mut().clear_curves();
    }));

    parent_box.pack_start(&box1, true, true, 0);

    app_state.borrow_mut().add_chart(chart_state1.clone());
}

/// Takes care of splitting the chart into two areas
/// This is done by determining the containing box parent,
/// and removing this box from the parent, placing a new
/// box in between in the box hierarchy..
fn split_chart(app_state: &GuiStateHandle, box1: &gtk::Box, orientation: gtk::Orientation) {
    // Create new split pane:
    let new_box = gtk::Box::new(orientation, 0);

    // Determine parent box (either vertical or horizontal)
    let parent_box: gtk::Box = box1.parent().unwrap().downcast::<gtk::Box>().unwrap();

    // find position in parent:
    let index = find_index(box1.clone(), &parent_box);
    info!("Old position: {:?}", index);

    // Remove original container and add new box:
    parent_box.remove(box1);
    parent_box.pack_start(&new_box, true, true, 0);
    parent_box.reorder_child(&new_box, index); // move to first position again!

    new_box.pack_start(box1, true, true, 0);

    // create new chart area:
    create_new_chart_area(&app_state, new_box.clone());

    new_box.show_all();
}

/// Complex way of finding position of box in parent box:
fn find_index(box1: gtk::Box, parent: &gtk::Box) -> i32 {
    for (index, child) in parent.children().iter().enumerate() {
        let child_box: gtk::Box = child.clone().downcast::<gtk::Box>().unwrap();
        if child_box == box1 {
            return index as i32;
        }
    }

    0
}

fn close_chart(box1: &gtk::Box) {
    // Determine parent box:
    let parent_box: gtk::Box = box1.parent().unwrap().downcast::<gtk::Box>().unwrap();

    // Remove self from parent:
    parent_box.remove(box1);

    // Parent now contains a single plot!
    if parent_box.children().is_empty() {
        if let Ok(grand_parent_box) = parent_box.parent().unwrap().downcast::<gtk::Box>() {
            grand_parent_box.remove(&parent_box);
        }
    }
}

/// Apply various settings to the drawing area.
fn massage_drawing_area(new_chart_area: &gtk::DrawingArea) {
    new_chart_area.show();
    new_chart_area.set_hexpand(true);
    new_chart_area.set_vexpand(true);
    new_chart_area.set_size_request(200, 200);

    new_chart_area.set_can_focus(true);
    new_chart_area.set_can_default(true);
    new_chart_area.set_sensitive(true);
    new_chart_area.set_receives_default(true);

    // Enable some events to allow scrolling by mouse.
    new_chart_area.add_events(gdk::EventMask::ENTER_NOTIFY_MASK);
    new_chart_area.add_events(gdk::EventMask::POINTER_MOTION_MASK);
    new_chart_area.add_events(gdk::EventMask::LEAVE_NOTIFY_MASK);
    new_chart_area.add_events(gdk::EventMask::BUTTON_PRESS_MASK);
    new_chart_area.add_events(gdk::EventMask::BUTTON_MOTION_MASK);
    new_chart_area.add_events(gdk::EventMask::BUTTON_RELEASE_MASK);
    new_chart_area.add_events(gdk::EventMask::FOCUS_CHANGE_MASK);
    new_chart_area.add_events(gdk::EventMask::STRUCTURE_MASK);
    new_chart_area.add_events(gdk::EventMask::EXPOSURE_MASK);
    new_chart_area.add_events(gdk::EventMask::SCROLL_MASK);
    new_chart_area.add_events(gdk::EventMask::KEY_PRESS_MASK);
}

fn setup_drawing_area(
    draw_area: gtk::DrawingArea,
    app_state: GuiStateHandle,
    chart_id: &str,
) -> ChartStateHandle {
    // Always get mouse pointer motion:
    draw_area.add_events(gdk::EventMask::ENTER_NOTIFY_MASK);
    draw_area.add_events(gdk::EventMask::POINTER_MOTION_MASK);
    draw_area.add_events(gdk::EventMask::LEAVE_NOTIFY_MASK);

    let db = { app_state.borrow().db.clone() };
    let perf_tracer = app_state.borrow().get_perf_tracer();

    let chart_state =
        ChartState::new(db, perf_tracer, app_state, draw_area.clone(), chart_id).into_handle();

    // Connect draw event:
    draw_area.connect_draw(
        clone!(@strong chart_state => move |_, c|  chart_state.borrow_mut().draw_on_canvas(c)  ),
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
            let mime_payload: String = data.text().expect("Must work!!").to_string();
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
        let pos = e.position();
        debug!("Mouse press! {:?}", pos);
        chart_state.borrow_mut().start_drag(pos.0, pos.1);
        w.grab_focus();
        Inhibit(false)
    }));

    draw_area.connect_leave_notify_event(clone!(@strong chart_state => move |_, _| {
        debug!("Mouse leave!");
        chart_state.borrow_mut().set_cursor(None);
        Inhibit(false)
    }));

    draw_area.connect_motion_notify_event(clone!(@strong chart_state => move |draw_area, event| {
        let mut chart = chart_state.borrow_mut();

        let pos = event.position();
        debug!("Mouse motion! {:?}", pos);
        chart.set_cursor(Some((pos.0, pos.1)));

        if event.state().contains(gdk::ModifierType::BUTTON1_MASK) {
            chart.move_drag(pos.0, pos.1);
        }

        draw_area.queue_draw();

        Inhibit(false)
    }));

    draw_area.connect_scroll_event(clone!(@strong chart_state => move |_, event| {
        let mut chart = chart_state.borrow_mut();
        debug!(
            "Scroll wheel event! {:?}, {:?}, {:?}",
            event,
            event.delta(),
            event.direction()
        );
        let pixel_x_pos = event.position().0;
        let around = Some(pixel_x_pos);
        match event.direction() {
            gdk::ScrollDirection::Up => {
                chart.zoom_in_horizontal(around);
            }
            gdk::ScrollDirection::Down => {
                chart.zoom_out_horizontal(around);
            }
            gdk::ScrollDirection::Left => {
                chart.pan_left();
            }
            gdk::ScrollDirection::Right => {
                chart.pan_right();
            }
            _ => {}
        }
        Inhibit(false)
    }));

    draw_area.connect_size_allocate(clone!(@strong chart_state => move |_, allocation| {
        let width: f64 = allocation.width() as f64;
        let height: f64 = allocation.height() as f64;
        chart_state.borrow_mut().resize(width, height);
    }));

    // Connect key event:
    draw_area.connect_key_press_event(clone!(@strong chart_state => move |_, key| {
            let mut chart = chart_state.borrow_mut();

            chart.disable_tailing();
            match key.keyval() {
                gdk::keys::constants::Up | gdk::keys::constants::w => {
                    chart.pan_up();
                }
                gdk::keys::constants::Down | gdk::keys::constants::s => {
                    chart.pan_down();
                }
                gdk::keys::constants::Left | gdk::keys::constants::a => {
                    chart.pan_left();
                }
                gdk::keys::constants::Right | gdk::keys::constants::d => {
                    chart.pan_right();
                }
                gdk::keys::constants::i => {
                    chart.zoom_in_vertical();
                }
                gdk::keys::constants::k => {
                    chart.zoom_out_vertical();
                }
                gdk::keys::constants::KP_Add | gdk::keys::constants::l => {
                    chart.zoom_in_horizontal(None);
                }
                gdk::keys::constants::KP_Subtract | gdk::keys::constants::j => {
                    chart.zoom_out_horizontal(None);
                }
                gdk::keys::constants::Home | gdk::keys::constants::Return => {
                    chart.zoom_fit();
                }
                gdk::keys::constants::BackSpace => {
                    chart.clear_curves();
                }
                gdk::keys::constants::_1 => {
                    chart.set_cursor1();
                }
                gdk::keys::constants::_2 => {
                    chart.set_cursor2();
                }
                other_key => {
                    println!("Key! {:?}", other_key);
                }
            };

            Inhibit(true)
        }
    ));

    chart_state
}
