use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use lognplot::tsdb::TsDbHandle;

use super::chart_widget::setup_drawing_area;
use super::io::{load_data_from_hdf5, save_data_as_hdf5};
use super::session::{load_session, save_session};
use super::signal_repository::setup_signal_repository;
use super::{GuiState, GuiStateHandle};

pub fn open_gui(db_handle: TsDbHandle) {
    let app_state = GuiState::new(db_handle).into_handle();

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

    // Connect the data set tree:
    setup_signal_repository(&builder, app_state.clone());

    setup_chart_area(&builder, app_state.clone());
    setup_menus(&builder, app_state.clone());
    setup_toolbar_buttons(&builder, app_state.clone());
    setup_notify_change(app_state);

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    if let Ok(Some(icon)) = crate::resources::load_icon() {
        window.set_icon(Some(&icon));
    }

    window.set_application(Some(app));
    window.show_all();
}

fn setup_chart_area(builder: &gtk::Builder, app_state: GuiStateHandle) {
    // First chart:
    let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
    let chart_state1 = setup_drawing_area(draw_area, app_state.clone(), "chart1");
    app_state.borrow_mut().add_chart(chart_state1);

    // Second chart:
    let draw_area2: gtk::DrawingArea = builder.get_object("chart_control2").unwrap();
    let chart_state2 = setup_drawing_area(draw_area2, app_state.clone(), "chart2");
    app_state.borrow_mut().add_chart(chart_state2);

    // Split handler:
    // TODO:
    // let hbx: gtk::Box = builder.get_object("root_splittor_box").unwrap();
    // let split_button: gtk::Button = builder.get_object("button_split_vertical").unwrap();
    // split_button.connect_clicked(move |_| {
    //     println!("Split!");
    //     hbx.remove(&draw_area2);
    //     let paned = gtk::Paned::new(gtk::Orientation::Vertical);
    //     hbx.add(&paned);
    //     paned.add(&draw_area2);

    //     // hbx.add_widget(draw_area2);
    // });
}

fn setup_about_dialog(about_dialog: &gtk::AboutDialog) {
    if let Ok(Some(icon)) = crate::resources::load_icon() {
        about_dialog.set_icon(Some(&icon));
    }
    if let Ok(Some(logo)) = crate::resources::load_logo() {
        about_dialog.set_logo(Some(&logo));
    }

    about_dialog.connect_delete_event(|p, _| gtk::Inhibit(p.hide_on_delete()));
    about_dialog.connect_response(|p, _| p.hide());
}

/// Construct new plot window with the given number of plots.
fn new_plot_window(app_state: GuiStateHandle, amount: usize) {
    info!("New window!");
    let chart_id = format!("chart{}", app_state.borrow().num_charts() + 1);
    let new_window = gtk::WindowBuilder::new()
        .type_(gtk::WindowType::Toplevel)
        .title(&format!("Lognplot {}", chart_id))
        .build();
    if let Ok(Some(icon)) = crate::resources::load_icon() {
        new_window.set_icon(Some(&icon));
    }

    let mut charts = vec![];
    if amount > 1 {
        let grid = gtk::Grid::new();
        new_window.add(&grid);

        // Create grid of plots:
        for row in 0_i32..amount as i32 {
            for column in 0_i32..amount as i32 {
                let chart_id = format!(
                    "chart{}",
                    app_state.borrow().num_charts() + 1 + charts.len()
                );
                // println!("Row: {}", row);
                let new_chart_area = gtk::DrawingArea::new();
                grid.attach(&new_chart_area, column, row, 1, 1);
                massage_drawing_area(&new_chart_area);

                let chart_n = setup_drawing_area(new_chart_area, app_state.clone(), &chart_id);
                charts.push(chart_n);
            }
        }

        grid.show();
    } else {
        let new_chart_area = gtk::DrawingArea::new();
        new_window.add(&new_chart_area);
        massage_drawing_area(&new_chart_area);

        let chart_n = setup_drawing_area(new_chart_area, app_state.clone(), &chart_id);
        charts.push(chart_n);
    }

    for chart_n in &charts {
        app_state.borrow_mut().add_chart(chart_n.clone());
    }

    new_window.connect_delete_event(clone!(@strong app_state => move |_, _| {
        // Remove all chart structs from the app state:
        for chart_n in &charts {
            app_state.borrow_mut().delete_chart(&chart_n);
        }
        Inhibit(false)
    }));
    new_window.show_all();
}

/// Apply various settings to the drawing area.
fn massage_drawing_area(new_chart_area: &gtk::DrawingArea) {
    new_chart_area.show();
    new_chart_area.set_hexpand(true);
    new_chart_area.set_vexpand(true);

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

fn setup_menus(builder: &gtk::Builder, app_state: GuiStateHandle) {
    let about_menu_item: gtk::MenuItem = builder.get_object("about_menu_item").unwrap();
    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();

    setup_about_dialog(&about_dialog);
    about_menu_item.connect_activate(move |_| {
        about_dialog.show();
    });

    let menu_new_plot_window: gtk::MenuItem = builder.get_object("menu_new_plot_window").unwrap();
    menu_new_plot_window.connect_activate(clone!(@strong app_state => move |_| {
        new_plot_window(app_state.clone(), 1);
    }));

    let menu_new_grid_plot_window: gtk::MenuItem =
        builder.get_object("menu_new_grid_plot_window").unwrap();
    menu_new_grid_plot_window.connect_activate(clone!(@strong app_state => move |_| {
        new_plot_window(app_state.clone(), 2);
    }));

    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let quit_menu: gtk::MenuItem = builder.get_object("menu_quit").unwrap();
    quit_menu.connect_activate(move |_| {
        top_level.close();
    });

    let menu_new: gtk::MenuItem = builder.get_object("menu_new").unwrap();
    menu_new.connect_activate(clone!(@strong app_state => move |_| {
        app_state.borrow().delete_all_data();
    }));

    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    menu_open.connect_activate(clone!(@strong app_state => move |_| {
        load_data_from_hdf5(&top_level, &app_state);
    }));

    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let menu_save: gtk::MenuItem = builder.get_object("menu_save").unwrap();
    menu_save.connect_activate(clone!(@strong app_state => move |_| {
        save_data_as_hdf5(&top_level, &app_state);
    }));

    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let menu_save_session: gtk::MenuItem = builder.get_object("menu_save_session").unwrap();
    menu_save_session.connect_activate(clone!(@strong app_state => move |_| {
        save_session(&top_level, &app_state);
    }));

    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let menu_load_session: gtk::MenuItem = builder.get_object("menu_load_session").unwrap();
    menu_load_session.connect_activate(clone!(@strong app_state => move |_| {
        load_session(&top_level, &app_state);
    }));
}

fn setup_toolbar_buttons(builder: &gtk::Builder, app_state: GuiStateHandle) {
    // Drop database:
    {
        let tb_delete_db: gtk::ToolButton = builder.get_object("tb_delete_all").unwrap();
        tb_delete_db.connect_clicked(clone!(@strong app_state => move |_tb| {
            app_state.borrow().delete_all_data();
        }));
    }

    // clear button:
    {
        let tb_clear_plot: gtk::ToolButton = builder.get_object("tb_clear_plot").unwrap();
        tb_clear_plot.connect_clicked(clone!(@strong app_state => move |_tb| {
            app_state.borrow_mut().clear_curves();
        }));
    }

    // zoom fit:
    {
        let tb_zoom_fit: gtk::ToolButton = builder.get_object("tb_zoom_fit").unwrap();
        tb_zoom_fit.connect_clicked(clone!(@strong app_state => move |_tb| {
            app_state.borrow_mut().zoom_fit();
        }));
    }

    // Zoom to button:
    {
        let tb_zoom_to: gtk::MenuToolButton = builder.get_object("tb_zoom_to").unwrap();
        let menu2: gtk::Menu = builder.get_object("my_menu1").unwrap();
        tb_zoom_to.set_menu(&menu2);

        let menu_ids = vec![
            ("menu_last_year", 365.0 * 24.0 * 60.0 * 60.0),
            ("menu_last_day", 24.0 * 60.0 * 60.0),
            ("menu_last_hour", 60.0 * 60.0),
            ("menu_last_10_minutes", 10.0 * 60.0),
            ("menu_last_minute", 60.0),
            ("menu_last_30_seconds", 30.0),
            ("menu_last_10_seconds", 10.0),
            ("menu_last_second", 1.0),
        ];
        for (menu_id, tail_duration) in menu_ids {
            let menu_item: gtk::MenuItem = builder.get_object(menu_id).unwrap();
            menu_item.connect_activate(clone!(@strong app_state => move |_tb| {
                info!("Zoom to last {} seconds", tail_duration);
                app_state
                    .borrow_mut()
                    .enable_tailing(tail_duration);
            }));
        }
    }

    {
        let tb_link_x_axis: gtk::ToggleToolButton = builder.get_object("tb_link_x_axis").unwrap();
        tb_link_x_axis.connect_toggled(clone!(@strong app_state => move |tb| {
            app_state.borrow_mut().set_linked_x_axis(tb.get_active());
        }));
    }
}

/// Subscribe to database changes and redraw correct things.
fn setup_notify_change(app_state: GuiStateHandle) {
    let mut receiver = app_state.borrow().db.new_notify_queue();

    // Insert async future function into the event loop:
    let main_context = glib::MainContext::default();
    main_context.spawn_local(async move {
        use futures::StreamExt;
        while let Some(event) = receiver.next().await {
            // println!("Event: {:?}", event);
            app_state.borrow().handle_event(&event);

            // Delay to emulate rate limiting of events.
            glib::timeout_future_with_priority(glib::Priority::default(), 200).await;

            // Re-query database for some extra samples:
            app_state.borrow().db.poll_events();
        }
    });
}
