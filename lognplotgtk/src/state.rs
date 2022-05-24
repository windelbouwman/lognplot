use crate::chart_state::{ChartState, ChartStateHandle};
use crate::session;
use lognplot::tracer::AnyTracer;
use lognplot::tsdb::{DataChangeEvent, TsDbHandle};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

/// Struct with some GUI state in it which will be shown in the GUI.
pub struct GuiState {
    pub db: TsDbHandle,
    perf_tracer: Arc<AnyTracer>,
    charts: Vec<ChartStateHandle>,
    link_x_axis: bool,
}

impl GuiState {
    pub fn new(db: TsDbHandle, perf_tracer: Arc<AnyTracer>) -> Self {
        // let perf_tracer = Arc::new(DbTracer::new(db.clone()));
        GuiState {
            db,
            perf_tracer,
            charts: vec![],
            link_x_axis: false,
        }
    }

    pub fn into_handle(self) -> GuiStateHandle {
        Rc::new(RefCell::new(self))
    }

    pub fn get_perf_tracer(&self) -> Arc<AnyTracer> {
        self.perf_tracer.clone()
    }

    pub fn delete_all_data(&self) {
        info!("Drop all data from database, to start a new!");
        self.db.delete_all();
    }

    #[cfg(feature = "hdf5")]
    pub fn save(&self, filename: &Path) -> Result<(), String> {
        info!("Save data to {:?}", filename);
        super::io::export_data(self.db.clone(), filename).map_err(|e| e.to_string())
    }

    #[cfg(feature = "hdf5")]
    pub fn load(&self, filename: &Path) -> Result<(), String> {
        super::io::import_data(self.db.clone(), filename).map_err(|e| e.to_string())
    }

    #[cfg(not(feature = "hdf5"))]
    pub fn load(&self, _filename: &Path) -> Result<(), String> {
        Err("No hdf5 support!".to_owned())
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
        self.charts
            .retain(|e| e.borrow().id() != chart.borrow().id());
    }

    pub fn num_charts(&self) -> usize {
        self.charts.len()
    }

    /// Add a curve with the given name to the first chart.
    ///
    /// Handy for double click / enter press on a signal.
    pub fn add_curve(&self, name: &str, chart_index: Option<usize>) {
        if let Some(index) = chart_index {
            if index > 0 && index <= self.charts.len() {
                self.charts[index - 1].borrow_mut().add_curve(name);
            }
        } else {
            self.charts.first().unwrap().borrow_mut().add_curve(name);
        };
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

    /// Call this from a timer to scroll to latest
    pub fn do_tailing(&self) {
        for chart in &self.charts {
            chart.borrow_mut().do_tailing();
        }
    }

    pub fn handle_event(&self, event: &DataChangeEvent) {
        for chart in &self.charts {
            chart.borrow_mut().handle_event(event);
        }
    }

    pub fn set_linked_x_axis(&mut self, link_axes: bool) {
        self.link_x_axis = link_axes;

        if link_axes {
            if let Some((first, rest)) = self.charts.split_first() {
                first.borrow_mut().disable_tailing();
                for chart in rest {
                    chart.borrow_mut().sync_x_axis(&first.borrow());
                }
            }
        }
    }

    pub fn sync_x_axis(&self, source_chart: &ChartState) {
        if self.link_x_axis {
            for chart in &self.charts {
                // If chart is already borrowed, this is the sync call originating chart!
                if let Ok(mut h) = chart.try_borrow_mut() {
                    h.sync_x_axis(source_chart);
                }
            }
        }
    }

    /// Distribute cursor position over other charts.
    pub fn sync_cursor(&self, source_chart: &ChartState) {
        for chart in &self.charts {
            // If chart is already borrowed, this is the sync call originating chart!
            if let Ok(mut h) = chart.try_borrow_mut() {
                h.sync_cursor(source_chart);
            }
        }
    }
}

pub type GuiStateHandle = Rc<RefCell<GuiState>>;

impl std::fmt::Display for GuiState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Db: {}", self.db)
    }
}
