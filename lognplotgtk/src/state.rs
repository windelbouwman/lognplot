use std::cell::RefCell;
use std::rc::Rc;

use lognplot::chart::{Chart, Curve, CurveData};
use lognplot::tsdb::TsDbHandle;

/// Struct with some GUI state in it which will be shown in the GUI.
pub struct GuiState {
    pub chart: Chart,
    signal_names: Vec<String>,
    db: TsDbHandle,
    // TODO:
    color_wheel: Vec<String>,
    color_index: usize,
    tailing: Option<f64>,
}

fn new_chart(db: TsDbHandle) -> Chart {
    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    chart
}

impl GuiState {
    pub fn new(db: TsDbHandle) -> Self {
        let chart = new_chart(db.clone());
        let color_wheel = vec!["blue".to_string(), "red".to_string(), "green".to_string()];
        GuiState {
            chart,
            signal_names: vec![],
            db,
            color_wheel,
            color_index: 0,
            tailing: None,
        }
    }

    pub fn into_handle(self) -> GuiStateHandle {
        Rc::new(RefCell::new(self))
    }

    pub fn get_new_signal_names(&mut self) -> Vec<String> {
        // Ugh, this function is wrong...
        let all_names = self.db.get_signal_names();
        let mut new_names = vec![];
        for name in all_names {
            if !self.signal_names.contains(&name) {
                self.signal_names.push(name.clone());
                new_names.push(name);
            }
        }
        new_names
    }

    pub fn add_curve(&mut self, name: &str) {
        // self.chart.add_curve(Curve::new());
        let tsdb_data = CurveData::trace(name, self.db.clone());
        let color = self.next_color();
        let curve2 = Curve::new(tsdb_data, &color);

        self.chart.add_curve(curve2);
        self.chart.autoscale();
    }

    pub fn next_color(&mut self) -> String {
        let color = self.color_wheel[self.color_index].clone();
        self.color_index += 1;
        if self.color_index >= self.color_wheel.len() {
            self.color_index = 0;
        }
        color
    }

    pub fn zoom_fit(&mut self) {
        self.disable_tailing();
        self.chart.autoscale();
    }

    pub fn clear_curves(&mut self) {
        self.disable_tailing();
        self.chart.clear_curves();
    }

    pub fn pan_left(&mut self) {
        info!("pan left!");
        self.disable_tailing();
        self.chart.pan_horizontal(-0.1);
        self.chart.fit_y_axis();
    }

    pub fn pan_right(&mut self) {
        info!("Pan right!");
        self.disable_tailing();
        self.chart.pan_horizontal(0.1);
        self.chart.fit_y_axis();
    }

    pub fn pan_up(&mut self) {
        info!("pan up!");
        self.disable_tailing();
        self.chart.pan_vertical(-0.1);
    }

    pub fn pan_down(&mut self) {
        info!("pan down!");
        self.disable_tailing();
        self.chart.pan_vertical(0.1);
    }

    pub fn zoom_in_horizontal(&mut self) {
        info!("Zoom in horizontal");
        self.disable_tailing();
        self.chart.zoom_horizontal(-0.1);
        self.chart.fit_y_axis();
    }

    pub fn zoom_out_horizontal(&mut self) {
        info!("Zoom out horizontal");
        self.disable_tailing();
        self.chart.zoom_horizontal(0.1);
        self.chart.fit_y_axis();
    }

    pub fn zoom_to_last(&mut self, tail_duration: f64) {
        self.chart.zoom_to_last(tail_duration);
        self.chart.fit_y_axis();
    }

    pub fn enable_tailing(&mut self, tail_duration: f64) {
        self.tailing = Some(tail_duration);
    }

    pub fn disable_tailing(&mut self) {
        self.tailing = None;
    }

    pub fn do_tailing(&mut self) -> bool {
        if let Some(x) = self.tailing {
            self.zoom_to_last(x);
            true
        } else {
            false
        }
    }
}

pub type GuiStateHandle = Rc<RefCell<GuiState>>;

impl std::fmt::Display for GuiState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Db: {}", self.db)
    }
}
