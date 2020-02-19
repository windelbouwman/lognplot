use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use crate::chart_widget::ChartStateHandle;
use crate::session;
use lognplot::time::TimeStamp;
use lognplot::tsdb::{
    Aggregation, DataChangeEvent, Observation, Sample, SampleMetrics, TsDbHandle,
};

/// Struct with some GUI state in it which will be shown in the GUI.
pub struct GuiState {
    gui_start_instant: Instant,
    pub db: TsDbHandle,
    charts: Vec<ChartStateHandle>,
}

impl GuiState {
    pub fn new(db: TsDbHandle) -> Self {
        GuiState {
            gui_start_instant: Instant::now(),
            db,
            charts: vec![],
        }
    }

    /// This is cool stuff, log metrics about render time for example to database itself :)
    pub fn log_meta_metric(&self, name: &str, timestamp: Instant, value: f64) {
        let elapsed = timestamp.duration_since(self.gui_start_instant);
        let elapsed_seconds: f64 = elapsed.as_secs_f64();
        let timestamp = TimeStamp::new(elapsed_seconds);
        let observation = Observation::new(timestamp, Sample::new(value));
        self.db.add_value(name, observation);
    }

    pub fn into_handle(self) -> GuiStateHandle {
        Rc::new(RefCell::new(self))
    }

    pub fn drop_data(&self) {
        info!("Drop all data from database, to start a new!");
        self.db.drop_all();
    }

    #[cfg(feature = "export-hdf5")]
    pub fn save(&self, filename: &Path) -> Result<(), String> {
        info!("Save data to {:?}", filename);
        super::io::export_data(self.db.clone(), filename).map_err(|e| e.to_string())
    }

    #[cfg(not(feature = "export-hdf5"))]
    pub fn save(&self, filename: &Path) -> Result<(), String> {
        let msg = format!(
            "Not saving to {:?}, since export-hdf5 feature is not enabled.",
            filename
        );
        Err(msg)
    }

    pub fn save_session(&self, filename: &Path) -> std::io::Result<()> {
        let mut s = session::Session::new();
        for chart in &self.charts {
            s.add_item(chart.borrow().get_session_item());
        }
        let f = std::fs::File::create(filename)?;
        serde_json::to_writer(f, &s)?;
        Ok(())
    }

    pub fn load_session(&mut self, filename: &Path) -> std::io::Result<()> {
        let f = std::fs::File::open(filename)?;
        let s: session::Session = serde_json::from_reader(f)?;
        for (chart, item) in self.charts.iter().zip(s.dashboard.iter()) {
            chart.borrow_mut().set_session_item(item);
        }
        Ok(())
    }

    pub fn get_signal_summary(&self, name: &str) -> Option<Aggregation<Sample, SampleMetrics>> {
        self.db.summary(name, None)
    }

    pub fn add_chart(&mut self, chart: ChartStateHandle) {
        self.charts.push(chart);
    }

    pub fn add_curve(&self, name: &str) {
        self.charts.first().unwrap().borrow_mut().add_curve(name);
    }

    pub fn zoom_fit(&self) {
        for chart in &self.charts {
            chart.borrow_mut().zoom_fit();
        }
    }

    pub fn pan_left(&self) {
        for chart in &self.charts {
            chart.borrow_mut().pan_left();
        }
    }

    pub fn pan_right(&self) {
        for chart in &self.charts {
            chart.borrow_mut().pan_right();
        }
    }

    pub fn clear_curves(&self) {
        for chart in &self.charts {
            chart.borrow_mut().clear_curves();
        }
    }

    pub fn enable_tailing(&self, tail_duration: f64) {
        for chart in &self.charts {
            chart.borrow_mut().enable_tailing(tail_duration);
        }
    }

    pub fn handle_event(&self, event: &DataChangeEvent) {
        for chart in &self.charts {
            chart.borrow().handle_event(event);
        }
    }
}

pub type GuiStateHandle = Rc<RefCell<GuiState>>;

impl std::fmt::Display for GuiState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Db: {}", self.db)
    }
}
