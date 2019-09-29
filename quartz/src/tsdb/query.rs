use super::db::TsDb;
use super::sample::Sample;
use crate::time::{Resolution, TimeSpan};

pub trait Query {
    fn get_values(&self, name: &str, interval: &TimeSpan, resolution: &Resolution) -> Vec<Sample>;
}

impl Query for TsDb {
    fn get_values(
        &self,
        name: &str,
        _interval: &TimeSpan,
        _resolution: &Resolution,
    ) -> Vec<Sample> {
        self.data.lock().unwrap().get(name).unwrap().to_vec()
    }
}
