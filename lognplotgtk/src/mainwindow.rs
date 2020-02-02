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

    setup_signal_repository(&builder, app_state.clone());

    let draw_area: gtk::DrawingArea = builder.get_object("chart_control").unwrap();
    setup_drawing_area(draw_area.clone(), app_state.clone());

    let about_menu_item: gtk::MenuItem = builder.get_object("about_menu_item").unwrap();
    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();

    about_menu_item.connect_activate(move |_m| {
        about_dialog.show();
    });

    let menu_save: gtk::MenuItem = builder.get_object("menu_save").unwrap();
    menu_save.connect_activate(clone!(@strong app_state => move |_m| {
        app_state.borrow().save();
    }));

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
            app_state.borrow_mut().clear_curves();
            draw_area.queue_draw();
        }));
    }

    // zoom fit:
    {
        let tb_zoom_fit: gtk::ToolButton = builder.get_object("tb_zoom_fit").unwrap();
        tb_zoom_fit.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            app_state.borrow_mut().zoom_fit();
            draw_area.queue_draw();
        }));
    }

    // pan left:
    {
        let tb_pan_left: gtk::ToolButton = builder.get_object("tb_pan_left").unwrap();
        tb_pan_left.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            app_state.borrow_mut().pan_left();
            draw_area.queue_draw();
        }));
    }

    // pan right:
    {
        let tb_pan_right: gtk::ToolButton = builder.get_object("tb_pan_right").unwrap();
        tb_pan_right.connect_clicked(clone!(@strong app_state, @strong draw_area => move |_tb| {
            app_state.borrow_mut().pan_right();
            draw_area.queue_draw();
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
