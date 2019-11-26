use std::cell::RefCell;
use std::rc::Rc;

use lognplot::chart::{Chart, Curve, CurveData};
use lognplot::tsdb::TsDbHandle;

/// Struct with some GUI state in it which will be shown in the GUI.
pub struct GuiState {
    pub chart: Chart,
}

fn new_chart(db: TsDbHandle) -> Chart {
    db.new_trace("Trace0");

    // Create data:
    // let mut x = vec![];
    // let mut y = vec![];
    // for i in 0..900 {
    //     let f = (i as f64) * 0.1;
    //     let v = 20.0 * f.sin() + f * 2.0;

    //     // Construct raw data:
    //     x.push(f);
    //     y.push(v);

    //     // Add observations:
    //     let timestamp = TimeStamp::new(f);
    //     let sample = Sample::new(v + 20.0);
    //     let observation = Observation::new(timestamp, sample);
    //     // db.add_value("bla", observation);
    // }

    // info!("Plotting len(x)= {:?} len(y)= {:?}", x.len(), y.len());

    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    // let curve_data = CurveData::points(x, y);
    // let curve = Curve::new(curve_data);
    // chart.add_curve(curve);

    let tsdb_data = CurveData::trace("Trace0", db);
    let curve2 = Curve::new(tsdb_data);

    chart.add_curve(curve2);
    chart.autoscale();
    chart
}

impl GuiState {
    pub fn new(db: TsDbHandle) -> Self {
        GuiState {
            chart: new_chart(db.clone()),
            // db,
        }
    }

    pub fn into_handle(self) -> GuiStateHandle {
        Rc::new(RefCell::new(self))
    }

    pub fn pan_left(&mut self) {
        info!("pan left!");
        self.chart.pan_horizontal(-0.1);
        self.chart.fit_y_axis();
    }

    pub fn pan_right(&mut self) {
        info!("Pan right!");
        self.chart.pan_horizontal(0.1);
        self.chart.fit_y_axis();
    }

    pub fn pan_up(&mut self) {
        info!("pan up!");
        self.chart.pan_vertical(-0.1);
    }

    pub fn pan_down(&mut self) {
        info!("pan down!");
        self.chart.pan_vertical(0.1);
    }

    pub fn zoom_in_horizontal(&mut self) {
        info!("Zoom in horizontal");
        self.chart.zoom_horizontal(-0.1);
        self.chart.fit_y_axis();
    }

    pub fn zoom_out_horizontal(&mut self) {
        info!("Zoom out horizontal");
        self.chart.zoom_horizontal(0.1);
        self.chart.fit_y_axis();
    }
}

pub type GuiStateHandle = Rc<RefCell<GuiState>>;
