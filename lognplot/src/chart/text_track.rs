use crate::time::TimeSpan;
use crate::tsdb::{CountMetrics, Query, RangeQueryResult, Text, TsDbHandle};

/// A single track with textual events.
pub struct TextTrack {
    name: String,
    db: TsDbHandle,
}

impl TextTrack {
    pub fn new(name: String, db: TsDbHandle) -> Self {
        TextTrack { name, db }
    }

    pub fn query(
        &self,
        timespan: &TimeSpan,
        amount: usize,
    ) -> Option<RangeQueryResult<Text, CountMetrics>> {
        let query = Query::create().amount(amount).span(&timespan).build();
        let result = self.db.query_text(&self.name, query);
        result.inner
    }
}
