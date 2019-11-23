//! Thread usable handle. Wrapper around a database.

use super::{Observation, Query, QueryResult, Sample, TsDb};
use std::sync::{Arc, Mutex};

pub type TsDbHandle = Arc<LockedTsDb>;

pub fn make_handle(db: TsDb) -> TsDbHandle {
    Arc::new(LockedTsDb::new(db))
}

#[derive(Debug)]
pub struct LockedTsDb {
    db: Mutex<TsDb>,
}

impl LockedTsDb {
    pub fn new(db: TsDb) -> Self {
        LockedTsDb { db: Mutex::new(db) }
    }

    pub fn new_trace(&self, name: &str) {
        self.db.lock().unwrap().new_trace(name)
    }

    pub fn add_values(&self, name: &str, samples: Vec<Observation<Sample>>) {
        self.db.lock().unwrap().add_values(name, samples);
    }

    /// Query the database.
    pub fn query(&self, name: &str, query: Query) -> QueryResult {
        self.db.lock().unwrap().query(name, query)
    }
}
