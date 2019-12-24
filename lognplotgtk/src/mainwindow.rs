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
    let filter_edit: gtk::SearchEntry = builder.get_object("signal_search_entry").unwrap();
    let name_column: gtk::TreeViewColumn = builder.get_object("column_name").unwrap();
    setup_signal_repository(tree_view, filter_edit, name_column, app_state.clone());

    let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
    setup_drawing_area(draw_area.clone(), app_state.clone());

    let about_menu_item: gtk::MenuItem = builder.get_object("about_menu_item").unwrap();
    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();

    about_menu_item.connect_activate(move |_m| {
        about_dialog.show();
    });

    setup_toolbar_buttons(&builder, &draw_area, app_state.clone());

    // Refreshing timer!
    let tick = clone!(@strong app_state, @strong draw_area => move || {
        // println!("TICK!!!");
        // let redraw =
        app_state.borrow_mut().do_tailing();
        // if redraw {
        // }
        draw_area.queue_draw();
        gtk::prelude::Continue(true)
    });
    gtk::timeout_add(100, tick);

    // Connect application to window:
    let window: gtk::Window = builder.get_object("top_unit").unwrap();

    window.set_application(Some(app));
    window.show_all();
}

fn setup_toolbar_buttons(
    builder: &gtk::Builder,
    draw_area: &gtk::DrawingArea,
    app_state: GuiStateHandle,
) {
    // clear button:
    {
        let tb_clear_plot: gtk::ToolButton = builder.get_object("tb_clear_plot").unwrap();
        tb_clear_plot.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            info!("Clear plot!");
            app_state.borrow_mut().clear_curves();
            draw_area.queue_draw();
        }));
    }

    // zoom fit:
    {
        let tb_zoom_fit: gtk::ToolButton = builder.get_object("tb_zoom_fit").unwrap();
        tb_zoom_fit.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            info!("zoom fit!");
            app_state.borrow_mut().zoom_fit();
            draw_area.queue_draw();
        }));
    }

    // pan left:
    {
        let tb_pan_left: gtk::ToolButton = builder.get_object("tb_pan_left").unwrap();
        tb_pan_left.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            info!("Pan left!");
            app_state.borrow_mut().pan_left();
            draw_area.queue_draw();
        }));
    }

    // pan right:
    {
        let tb_pan_right: gtk::ToolButton = builder.get_object("tb_pan_right").unwrap();
        tb_pan_right.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            info!("Pan right!");
            app_state.borrow_mut().pan_right();
            draw_area.queue_draw();
        }));
    }

    // Zoom to button:
    {
        let tail_menu = gtk::Menu::new();

        let menu_item = gtk::MenuItem::new_with_label("Bla1 1");
        // menu_item.set_label("X bla dir");
        tail_menu.append(&menu_item);

        let menu_item2 = gtk::MenuItem::new();
        menu_item2.set_label("X gompie");
        tail_menu.append(&menu_item2);

        // tail_menu.add
        let tb_zoom_to: gtk::MenuToolButton = builder.get_object("tb_zoom_to").unwrap();
        let menu2: gtk::Menu = builder.get_object("my_menu1").unwrap();
        // tb_zoom_to.set_menu(&tail_menu);
        tb_zoom_to.set_menu(&menu2);

        let app_state_tail_button = app_state.clone();
        tb_zoom_to.connect_clicked(move |_tb| {
            let tail_duration = 60.0;
            info!("Zoom to last {} seconds", tail_duration);
            app_state_tail_button
                .borrow_mut()
                .enable_tailing(tail_duration);
        });
    }
}
