use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use lognplot::tsdb::TsDbHandle;
use lognplot::tsdb::{ChangeSubscriber, DataChangeEvent};

use super::chart_widget::setup_drawing_area;
use super::signal_repository::{setup_signal_repository, SignalBrowser};

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
    let app_state_add_curve = app_state.clone();
    let add_curve = move |name: &str| {
        app_state_add_curve.borrow_mut().add_curve(name);
    };
    let db = { app_state.borrow().db.clone() };

    let signal_pane = setup_signal_repository(&builder, db.clone(), add_curve);

    setup_chart_area(&builder, app_state.clone(), db.clone());
    setup_menus(&builder, app_state.clone());
    setup_toolbar_buttons(&builder, app_state.clone());
    setup_notify_change(app_state, signal_pane);

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    if let Ok(Some(icon)) = crate::resources::load_icon() {
        window.set_icon(Some(&icon));
    }

    window.set_application(Some(app));
    window.show_all();
}

fn setup_chart_area(builder: &gtk::Builder, app_state: GuiStateHandle, db: TsDbHandle) {
    // First chart:
    let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
    let chart_state1 = setup_drawing_area(draw_area, db.clone(), "chart1");
    app_state.borrow_mut().add_chart(chart_state1);

    // Second chart:
    let draw_area2: gtk::DrawingArea = builder.get_object("chart_control2").unwrap();
    let chart_state2 = setup_drawing_area(draw_area2.clone(), db, "chart2");
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
        .can_focus(true)
        .can_default(true)
        .has_focus(true)
        .has_default(true)
        .is_focus(true)
        .receives_default(true)
        .sensitive(true)
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
                let chart_id = format!("chart{}", app_state.borrow().num_charts() + 1);
                // println!("Row: {}", row);
                let new_chart_area = gtk::DrawingArea::new();
                grid.attach(&new_chart_area, column, row, 1, 1);
                massage_drawing_area(&new_chart_area);

                let chart_n =
                    setup_drawing_area(new_chart_area, app_state.borrow().db.clone(), &chart_id);
                charts.push(chart_n);
            }
        }

        grid.show();
    } else {
        let new_chart_area = gtk::DrawingArea::new();
        new_window.add(&new_chart_area);
        massage_drawing_area(&new_chart_area);

        let chart_n = setup_drawing_area(new_chart_area, app_state.borrow().db.clone(), &chart_id);
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
        app_state.borrow().drop_data();
    }));

    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    menu_open.set_sensitive(false);
    menu_open.connect_activate(move |_| {
        unimplemented!("TODO: implement open");
    });

    let top_level: gtk::Window = builder.get_object("top_unit").unwrap();
    let menu_save: gtk::MenuItem = builder.get_object("menu_save").unwrap();
    menu_save.set_sensitive(cfg!(feature = "export-hdf5"));
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

/// Popup a dialog and export data as HDF5 format.
fn save_data_as_hdf5(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Export data as HDF5"),
        Some(top_level),
        gtk::FileChooserAction::Save,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Save", gtk::ResponseType::Accept),
        ],
    );
    let res = dialog.run();
    let filename = dialog.get_filename();
    dialog.destroy();
    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Saving data to filename: {:?}", filename);
            let res = { app_state.borrow().save(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error saving data: {}", err);
                error!("{}", error_message);
                show_error(top_level, &error_message);
            } else {
                info!("Data saved success");
            }
        }
    }
}

/// Popup a dialog to save session for later usage.
fn save_session(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Export session as JSON"),
        Some(top_level),
        gtk::FileChooserAction::Save,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Save", gtk::ResponseType::Accept),
        ],
    );
    let res = dialog.run();
    let filename = dialog.get_filename();
    dialog.destroy();

    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Saving session to filename: {:?}", filename);
            let res = { app_state.borrow().save_session(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error saving session to {:?}: {}", filename, err);
                error!("{}", error_message);
                show_error(top_level, &error_message);
            } else {
                info!("Session saved!");
            }
        }
    }
}

/// Popup a dialog to restore a session from before.
fn load_session(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Import session from JSON file"),
        Some(top_level),
        gtk::FileChooserAction::Open,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept),
        ],
    );

    let res = dialog.run();
    let filename = dialog.get_filename();
    dialog.destroy();
    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Loading session to filename: {:?}", filename);
            let res = { app_state.borrow_mut().load_session(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error loading session from {:?}: {}", filename, err);
                error!("{}", error_message);
                show_error(top_level, &error_message);
            } else {
                info!("Session loaded!");
            }
        }
    }
}

fn show_error(top_level: &gtk::Window, message: &str) {
    let error_dialog = gtk::MessageDialog::new(
        Some(top_level),
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        &message,
    );
    error_dialog.run();
    error_dialog.destroy();
}

fn setup_toolbar_buttons(builder: &gtk::Builder, app_state: GuiStateHandle) {
    // Drop database:
    {
        let tb_drop_db: gtk::ToolButton = builder.get_object("tb_drop_all").unwrap();
        tb_drop_db.connect_clicked(clone!(@strong app_state => move |_tb| {
            app_state.borrow().drop_data();
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
}

/// Subscribe to database changes and redraw correct things.
fn setup_notify_change(app_state: GuiStateHandle, mut signal_pane: SignalBrowser) {
    // Register change handler:
    let (sender, mut receiver) = futures::channel::mpsc::channel::<DataChangeEvent>(0);
    {
        let db_handle = app_state.borrow_mut().db.clone();
        let sub = ChangeSubscriber::new(sender);
        db_handle.register_notifier(sub);
    }

    // Insert async future function into the event loop:
    let main_context = glib::MainContext::default();
    main_context.spawn_local(async move {
        use futures::StreamExt;
        while let Some(event) = receiver.next().await {
            // println!("Event: {:?}", event);
            signal_pane.handle_event(&event);
            app_state.borrow().handle_event(&event);

            // Delay to emulate rate limiting of events.
            glib::timeout_future_with_priority(glib::Priority::default(), 100).await;

            // Re-query database for some extra samples:
            app_state.borrow().db.poll_events();
        }
    });
}
