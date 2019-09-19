use super::db::TsDb;
use super::sample::Sample;
use crate::time::{Resolution, TimeSpan};

pub trait Query {
    fn get_values(&self, interval: &TimeSpan, resolution: &Resolution) -> Vec<Sample>;
}

impl Query for TsDb {
    fn get_values(&self, interval: &TimeSpan, resolution: &Resolution) -> Vec<Sample> {
        self.data.get("foo").unwrap().to_vec()
    }
}
