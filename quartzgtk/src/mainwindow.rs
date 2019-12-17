use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use lognplot::tsdb::TsDbHandle;

use super::chart_widget::setup_drawing_area;
use super::signal_repository::setup_signal_repository;

use super::{GuiState, GuiStateHandle};

pub fn open_gui(db_handle: TsDbHandle) {
    let app_state = GuiState::new(db_handle.clone()).into_handle();

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
    let tree_view: gtk::TreeView = builder.get_object("signal_tree_view").unwrap();
    let name_column: gtk::TreeViewColumn = builder.get_object("column_name").unwrap();
    setup_signal_repository(tree_view, name_column, app_state.clone());

    let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
    setup_drawing_area(draw_area.clone(), app_state.clone());

    let tb_clear_plot: gtk::ToolButton = builder.get_object("tb_clear_plot").unwrap();
    let app_state_clear_button = app_state.clone();
    let clear_button_draw_area = draw_area.clone();
    tb_clear_plot.connect_clicked(move |_tb| {
        println!("Clear plot!");
        app_state_clear_button.borrow_mut().chart.clear_curves();
        clear_button_draw_area.queue_draw();
    });

    let tb_zoom_to: gtk::ToolButton = builder.get_object("tb_zoom_to").unwrap();
    let app_state_tail_button = app_state.clone();
    tb_zoom_to.connect_clicked(move |_tb| {
        let tail_duration = 60.0;
        println!("Zoom to last {} seconds", tail_duration);
        app_state_tail_button
            .borrow_mut()
            .enable_tailing(tail_duration);
    });

    // Refreshing timer!
    let tick_app_state = app_state.clone();
    let tick_draw_area = draw_area.clone();
    let tick = move || {
        // println!("TICK!!!");
        if let Some(tail_duration) = tick_app_state.borrow().tail_duration() {
            tick_app_state.borrow_mut().zoom_to_last(tail_duration);
            tick_draw_area.queue_draw();
        }
        gtk::prelude::Continue(true)
    };
    gtk::timeout_add(100, tick);

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    window.set_application(Some(app));
    window.show_all();
}

// fn setup_toolbar_buttons()
