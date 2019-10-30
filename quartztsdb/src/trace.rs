//! The core idea of the time series database:
//! split the sample sequence into batches.
//! This will result in a tree of chunks, each chunk having either sub chunks
//! or leave chunks, with real data.
//! Also: keep track of certain metrics, such as min, max and sum.

// use std::cell::RefCell;
use super::chunk::Chunk;
use super::sample::Sample;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Trace {
    chunk: Mutex<Chunk>,
    num_points: Mutex<usize>,
}

impl Trace {
    pub fn add_values(&self, samples: Vec<Sample>) {
        for sample in samples {
            self.push(sample);
        }
    }

    pub fn push(&self, value: Sample) {
        self.chunk.lock().unwrap().push(value);
        *self.num_points.lock().unwrap() += 1;
    }

    pub fn to_vec(&self) -> Vec<Sample> {
        self.chunk.lock().unwrap().to_vec()
    }

    pub fn len(&self) -> usize {
        *self.num_points.lock().unwrap()
    }
}

impl Default for Trace {
    fn default() -> Self {
        let chunk = Default::default();

        Self {
            chunk,
            num_points: Mutex::new(0),
        }
    }
}
