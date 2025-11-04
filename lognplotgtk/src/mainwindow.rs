use super::chart_widget::create_new_chart_area;
use super::io::{load_data_from_hdf5, save_data_as_hdf5};
use super::session::{load_session, save_session};
use super::signal_repository::setup_signal_repository;
use super::{GuiState, GuiStateHandle};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use lognplot::tracer::AnyTracer;
use lognplot::tsdb::TsDbHandle;
use std::sync::Arc;

pub fn open_gui(db_handle: TsDbHandle, perf_tracer: Arc<AnyTracer>) {
    info!("Opening GUI");
    let app_state = GuiState::new(db_handle, perf_tracer).into_handle();
    let app_id = "com.github.windelbouwman.quartz";
    let application = Application::builder().application_id(app_id).build();
    application.connect_activate(move |app| build_ui(app, app_state.clone()));
    let args: Vec<String> = vec![];
    application.run_with_args(&args);
}

fn build_ui(app: &gtk::Application, app_state: GuiStateHandle) {
    info!("Setting up GUI elements");
    let window = gtk::Window::builder()
        .application(app)
        .title("Lognplot GTK GUI")
        .icon_name("lognplot-icon")
        .default_width(1400)
        .default_height(1000)
        .build();
    let main_pane = gtk::Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let signal_vbox = setup_signal_repository(&app_state);
    let chart_grid = create_chart_grid(&app_state);
    main_pane.set_start_child(Some(&signal_vbox));
    main_pane.set_end_child(Some(&chart_grid));
    let top_vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let menu_bar = create_menu_bar(&window, app_state.clone());
    top_vbox.append(&menu_bar);
    top_vbox.append(&main_pane);
    window.set_child(Some(&top_vbox));
    setup_tailing_timer(app_state.clone());
    setup_notify_change(app_state);
    window.present();
}

#[derive(Clone)]
enum LayoutVariation {
    _1x1,
    _1x2,
    _2x1,
    _2x2,
    _3x2,
}

#[derive(Clone)]
struct PlotSet {
    c1: gtk::Box,
    c2: gtk::Box,
    c3: gtk::Box,
    c4: gtk::Box,
    c5: gtk::Box,
    c6: gtk::Box,
}

impl PlotSet {
    fn show_variation(&self, var: &LayoutVariation) {
        match var {
            LayoutVariation::_1x1 => {
                self.c1.set_visible(true);
                self.c2.set_visible(false);
                self.c3.set_visible(false);
                self.c4.set_visible(false);
                self.c5.set_visible(false);
                self.c6.set_visible(false);
            }
            LayoutVariation::_1x2 => {
                self.c1.set_visible(true);
                self.c2.set_visible(true);
                self.c3.set_visible(false);
                self.c4.set_visible(false);
                self.c5.set_visible(false);
                self.c6.set_visible(false);
            }
            LayoutVariation::_2x1 => {
                self.c1.set_visible(true);
                self.c2.set_visible(false);
                self.c3.set_visible(true);
                self.c4.set_visible(false);
                self.c5.set_visible(false);
                self.c6.set_visible(false);
            }
            LayoutVariation::_2x2 => {
                self.c1.set_visible(true);
                self.c2.set_visible(true);
                self.c3.set_visible(true);
                self.c4.set_visible(true);
                self.c5.set_visible(false);
                self.c6.set_visible(false);
            }
            LayoutVariation::_3x2 => {
                self.c1.set_visible(true);
                self.c2.set_visible(true);
                self.c3.set_visible(true);
                self.c4.set_visible(true);
                self.c5.set_visible(true);
                self.c6.set_visible(true);
            }
        }
    }
}

fn create_chart_grid(app_state: &GuiStateHandle) -> gtk::Grid {
    let chart_grid = gtk::Grid::new();
    let c1 = create_new_chart_area(app_state);
    let c2 = create_new_chart_area(app_state);
    let c3 = create_new_chart_area(app_state);
    let c4 = create_new_chart_area(app_state);
    let c5 = create_new_chart_area(app_state);
    let c6 = create_new_chart_area(app_state);
    chart_grid.attach(&c1, 0, 0, 1, 1);
    chart_grid.attach(&c2, 1, 0, 1, 1);
    chart_grid.attach(&c3, 0, 1, 1, 1);
    chart_grid.attach(&c4, 1, 1, 1, 1);
    chart_grid.attach(&c5, 2, 0, 1, 1);
    chart_grid.attach(&c6, 2, 1, 1, 1);

    let plot_set = PlotSet {
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
    };
    plot_set.show_variation(&LayoutVariation::_1x1);

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let grid_options_menu = gtk::MenuButton::builder().label("Plot layout").build();
    let pop_over = gtk::Popover::builder().build();
    grid_options_menu.set_popover(Some(&pop_over));
    hbox.append(&grid_options_menu);
    chart_grid.attach(&hbox, 0, 2, 2, 1);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    pop_over.set_child(Some(&vbox));
    let variations = vec![
        LayoutVariation::_1x1,
        LayoutVariation::_1x2,
        LayoutVariation::_2x1,
        LayoutVariation::_2x2,
        LayoutVariation::_3x2,
    ];

    for var in &variations {
        let name = match var {
            LayoutVariation::_1x1 => "1x1",
            LayoutVariation::_1x2 => "1x2",
            LayoutVariation::_2x1 => "2x1",
            LayoutVariation::_2x2 => "2x2",
            LayoutVariation::_3x2 => "3x2",
        };
        let grid_option_button = gtk::Button::builder().label(name).build();
        vbox.append(&grid_option_button);
        grid_option_button.connect_clicked(clone!(
            #[strong]
            var,
            #[strong]
            pop_over,
            #[strong]
            plot_set,
            move |_button| {
                pop_over.hide();
                plot_set.show_variation(&var);
            }
        ));
    }

    chart_grid
}

fn setup_about_dialog(_top_level: &gtk::Window) -> gtk::AboutDialog {
    let about_dialog = gtk::AboutDialog::builder()
        .hide_on_close(true)
        .modal(true)
        .build();
    about_dialog.set_comments(Some("Lognplot GTK gui. This tool can be used to visualize incoming data from a real-time system."));
    about_dialog.set_website(Some("https://github.com/windelbouwman/lognplot"));
    about_dialog.set_website_label("Github website");
    about_dialog.set_license(Some("GPL 3.0"));
    about_dialog.set_authors(&["Windel Bouwman"]);
    about_dialog
}

/// Construct new plot window.
fn new_plot_window(app_state: GuiStateHandle) {
    info!("New window!");
    let chart_id = format!("chart{}", app_state.borrow().num_charts() + 1);
    let new_window = gtk::Window::builder()
        // .type_(gtk::WindowType::Toplevel)
        .title(&format!("Lognplot {}", chart_id))
        .build();
    let chart_grid = create_chart_grid(&app_state);
    new_window.set_child(Some(&chart_grid));
    new_window.present();
}

fn create_menu_bar(top_level: &gtk::Window, app_state: GuiStateHandle) -> gtk::Box {
    let menu_bar = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let about_button = gtk::Button::builder().label("About").build();
    let about_dialog = setup_about_dialog(top_level);
    menu_bar.append(&about_button);

    about_button.connect_clicked(move |_| {
        info!("Showing about dialog");
        about_dialog.present();
    });

    let menu_new_plot_window = gtk::Button::builder().label("new plot window").build();
    menu_bar.append(&menu_new_plot_window);
    menu_new_plot_window.connect_clicked(clone!(
        #[strong]
        app_state,
        move |_| {
            info!("Creating new plot window");
            new_plot_window(app_state.clone());
        }
    ));

    if cfg!(feature = "hdf5") {
        let menu_open = gtk::Button::builder().label("Open").build();
        menu_bar.append(&menu_open);
        menu_open.connect_clicked(clone!(
            #[strong]
            top_level,
            #[strong]
            app_state,
            move |_button| {
                load_data_from_hdf5(&top_level, &app_state);
            }
        ));
    }

    if cfg!(feature = "hdf5") {
        let menu_save = gtk::Button::builder().label("Save").build();
        menu_bar.append(&menu_save);
        menu_save.connect_clicked(clone!(
            #[strong]
            top_level,
            #[strong]
            app_state,
            move |_button| {
                save_data_as_hdf5(&top_level, &app_state);
            }
        ));
    }

    let menu_save_session = gtk::Button::builder().label("Save session").build();
    menu_bar.append(&menu_save_session);
    menu_save_session.connect_clicked(clone!(
        #[strong]
        top_level,
        #[strong]
        app_state,
        move |_| {
            save_session(&top_level, &app_state);
        }
    ));

    let menu_load_session = gtk::Button::builder().label("Load session").build();
    menu_bar.append(&menu_load_session);
    menu_load_session.connect_clicked(clone!(
        #[strong]
        top_level,
        #[strong]
        app_state,
        move |_| {
            load_session(&top_level, &app_state);
        }
    ));

    setup_toolbar_buttons(&menu_bar, app_state);

    menu_bar
}

fn setup_toolbar_buttons(menu_bar: &gtk::Box, app_state: GuiStateHandle) {
    // Drop database:
    let tb_delete_db = gtk::Button::builder().label("Clear history").build();
    menu_bar.append(&tb_delete_db);
    tb_delete_db.connect_clicked(clone!(
        #[strong]
        app_state,
        move |_button| {
            app_state.borrow().delete_all_data();
        }
    ));

    // clear button:
    let tb_clear_plot = gtk::Button::builder().label("Clear plot").build();
    menu_bar.append(&tb_clear_plot);
    tb_clear_plot.connect_clicked(clone!(
        #[strong]
        app_state,
        move |_button| {
            app_state.borrow_mut().clear_curves();
        }
    ));

    // zoom fit:
    let tb_zoom_fit = gtk::Button::builder().label("Zoom fit").build();
    menu_bar.append(&tb_zoom_fit);
    tb_zoom_fit.connect_clicked(clone!(
        #[strong]
        app_state,
        move |_button| {
            app_state.borrow_mut().zoom_fit();
        }
    ));

    let zoom_to = setup_zoom_to_options(app_state.clone());
    menu_bar.append(&zoom_to);
}

/// Setup zoom-to button and popover menu
fn setup_zoom_to_options(app_state: GuiStateHandle) -> gtk::MenuButton {
    let tb_zoom_to = gtk::MenuButton::builder().label("Follow last..").build();
    let pop_over = gtk::Popover::builder().build();
    tb_zoom_to.set_popover(Some(&pop_over));

    let menu_ids = vec![
        ("Last year", 365.0 * 24.0 * 60.0 * 60.0),
        ("Last day", 24.0 * 60.0 * 60.0),
        ("Last hour", 60.0 * 60.0),
        ("Last 10 minutes", 10.0 * 60.0),
        ("Last minute", 60.0),
        ("Last 30 seconds", 30.0),
        ("Last 10 seconds", 10.0),
        ("Last second", 1.0),
    ];
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    pop_over.set_child(Some(&vbox));
    for (menu_id, tail_duration) in menu_ids {
        let duration_button = gtk::Button::builder().label(menu_id).build();
        vbox.append(&duration_button);
        duration_button.connect_clicked(clone!(
            #[strong]
            app_state,
            #[strong]
            pop_over,
            move |_tb| {
                pop_over.hide();
                info!("Zoom to last {} seconds", tail_duration);
                app_state.borrow_mut().enable_tailing(tail_duration);
            }
        ));
    }
    tb_zoom_to
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
            glib::timeout_future_with_priority(
                glib::Priority::default(),
                std::time::Duration::from_millis(200),
            )
            .await;

            // Re-query database for some extra samples:
            app_state.borrow().db.poll_events();
        }
    });
}

/// Setup a timer to implement tailing of signals.
fn setup_tailing_timer(app_state: GuiStateHandle) {
    // Refreshing timer!
    let tick = move || {
        app_state.borrow_mut().do_tailing();
        glib::ControlFlow::Continue
    };
    glib::timeout_add_local(std::time::Duration::from_millis(57), tick);
}
