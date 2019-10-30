use super::db::TsDb;

impl Connection for TsDb {
    /// Open database
    fn open(&self) {
        info!("opening {}", self);
    }

    /// Close database
    fn close(&self) {
        trace!("closing {}", self);
    }
}

pub trait Connection {
    fn open(&self);
    fn close(&self);
}
