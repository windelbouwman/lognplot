use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::chart_widget::ChartStateHandle;
use crate::session;
use lognplot::tsdb::{DataChangeEvent, TsDbHandle};

/// Struct with some GUI state in it which will be shown in the GUI.
pub struct GuiState {
    pub db: TsDbHandle,
    charts: Vec<ChartStateHandle>,
}

impl GuiState {
    pub fn new(db: TsDbHandle) -> Self {
        GuiState { db, charts: vec![] }
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

    pub fn add_chart(&mut self, chart: ChartStateHandle) {
        self.charts.push(chart);
    }

    pub fn delete_chart(&mut self, chart: &ChartStateHandle) {
        // TODO?
        // self.charts.remove_item(&chart);
    }

    pub fn num_charts(&self) -> usize {
        self.charts.len()
    }

    pub fn add_curve(&self, name: &str) {
        self.charts.first().unwrap().borrow_mut().add_curve(name);
    }

    pub fn zoom_fit(&self) {
        for chart in &self.charts {
            chart.borrow_mut().zoom_fit();
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
