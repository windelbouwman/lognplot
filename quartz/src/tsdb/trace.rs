//! The core idea of the time series database:
//! split the sample sequence into batches.
//! This will result in a tree of chunks, each chunk having either sub chunks
//! or leave chunks, with real data.
//! Also: keep track of certain metrics, such as min, max and sum.

use super::chunk::Chunk;
use super::sample::Sample;

pub struct Trace {
    chunk: Chunk,
}

impl Trace {
    pub fn push(&mut self, value: Sample) {
        self.chunk.push(value);
    }

    pub fn to_vec(&self) -> Vec<Sample> {
        self.chunk.to_vec()
    }
}

impl Default for Trace {
    fn default() -> Self {
        let chunk = Default::default();

        Self { chunk }
    }
}
