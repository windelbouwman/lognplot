//! Implementation of a B-tree like data structure.
//!
//! The idea is to create leave nodes and intermediate nodes.
//! Leave and intermediate nodes can have multiple child nodes.

use super::metrics::SampleMetrics;
use super::sample::Sample;
use super::time::TimeSpan;

use std::cell::{RefCell, RefMut};

/// This implements a b-tree structure.
#[derive(Debug)]
pub struct Btree {
    root: RefCell<Chunk>,
}

/// Create an empty b-tree
impl Default for Btree {
    fn default() -> Self {
        let root = RefCell::new(Chunk::new_leave());
        Btree { root }
    }
}

impl Btree {
    /// Insert a sample into the tree
    pub fn insert(&self, sample: Sample) {
        // Strategy, traverse down, until a leave, and split on the way back upwards if
        // required.
        // Find proper chunk, or create one if required.
        let optionally_root_split = self.root.borrow_mut().insert(sample);

        if let Some(root_sibling) = optionally_root_split {
            let new_root = Chunk::new_intermediate();
            let old_root = self.root.replace(new_root);
            self.root.borrow_mut().add_chunk(old_root);
            self.root.borrow_mut().add_chunk(root_sibling);
        }
    }

    pub fn query_range(&self, timespan: &TimeSpan) -> Vec<Sample> {
        vec![]
    }

    pub fn to_vec(&self) -> Vec<Sample> {
        self.root.borrow().to_vec()
    }

    pub fn len(&self) -> usize {
        self.root.borrow().len()
    }
}

/// This constant defines the fanout ratio.
/// Each leave contains maximum this number of values
/// Also, each intermediate node also contains this maximum number
/// of subchunks.
const CHUNK_SIZE: usize = 32;

/// This is a sort of B+ tree data structure
/// to store a sequence of sample along with some
/// metrics about those samples.
#[derive(Debug)]
pub enum Chunk {
    /// An intermediate chunk with some leave
    /// chunk.
    SubChunk {
        chunks: Vec<Chunk>,
        metrics: SampleMetrics,
    },

    /// A leave chunk with some samples in it.
    Leave {
        samples: Vec<Sample>,
        metrics: SampleMetrics,
    },
}

/// Intermediate node
pub struct Branch {
    chunks: Vec<Chunk>,
    metrics: SampleMetrics,
}

/// Leave node type
pub struct Leave {
    samples: Vec<Sample>,
    metrics: SampleMetrics,
}

impl Chunk {
    pub fn new_intermediate() -> Self {
        Chunk::SubChunk {
            chunks: Vec::with_capacity(CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    /// Create a new leave chunk!
    pub fn new_leave() -> Self {
        Chunk::Leave {
            samples: Vec::with_capacity(CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    /// Test if this chunk is full
    pub fn is_full(&self) -> bool {
        match self {
            Chunk::Leave { samples, .. } => samples.len() >= CHUNK_SIZE,
            Chunk::SubChunk { chunks, .. } => chunks.len() >= CHUNK_SIZE,
        }
    }

    /// The insert into database operation!
    pub fn insert(&mut self, sample: Sample) -> Option<Chunk> {
        match self {
            Chunk::SubChunk { chunks, metrics } => {
                // For now alway insert into last chunk:
                let optional_new_chunk = chunks.last_mut().unwrap().insert(sample);

                // Optionally we have a new chunk which must be added.
                if let Some(new_chunk) = optional_new_chunk {
                    if chunks.len() < CHUNK_SIZE {
                        chunks.push(new_chunk);
                        None
                    } else {
                        // Split required!
                        // for now, just split by creating a new node.
                        debug!("Split of sub chunk node");
                        let mut new_sibling_chunk = Chunk::new_intermediate();
                        new_sibling_chunk.add_chunk(new_chunk);
                        Some(new_sibling_chunk)
                    }
                } else {
                    None
                }
            }
            Chunk::Leave { samples, metrics } => {
                if samples.len() < CHUNK_SIZE {
                    metrics.update(&sample);
                    samples.push(sample);
                    None
                } else {
                    // We must split!
                    debug!("Split of leave node!");
                    let mut new_leave = Chunk::new_leave();
                    new_leave.add_sample(sample);
                    Some(new_leave)
                }
            }
        }
    }

    /// Inject a chunk into this chunk.
    /// Note: chunk must be of variant subchunk, otherwise this
    /// will fail.
    pub fn add_chunk(&mut self, chunk: Chunk) {
        match self {
            Chunk::SubChunk { chunks, metrics } => {
                assert!(chunks.len() < CHUNK_SIZE);
                chunks.push(chunk);
            }
            x => {
                panic!("Wrong chunk type {:?} for add_chunk", x);
            }
        }
    }

    pub fn add_sample(&mut self, sample: Sample) {
        match self {
            Chunk::Leave { samples, metrics } => {
                assert!(samples.len() < CHUNK_SIZE);
                metrics.update(&sample);
                samples.push(sample);
            }
            x => {
                panic!("Wrong chunk type {:?} for add_sample", x);
            }
        }
    }

    /// Insert opertaion!
    // pub fn push(&mut self, value: Sample) {
    //     self.fold_into_metric(&value);
    //     match self {
    //         Chunk::SubChunk { chunks, .. } => {
    //             chunks.last_mut().unwrap().push(value);
    //         }
    //         Chunk::Leave { samples, .. } => {
    //             samples.push(value);
    //         }
    //     }
    // }

    /// Incorporate given sample into metrics.
    // pub fn fold_into_metric(&mut self, value: &Sample) {
    //     match self {
    //         Chunk::SubChunk { metrics, .. } | Chunk::Leave { metrics, .. } => {
    //             metrics.update(&value);
    //         }
    //     }
    // }

    /// Get all samples from this chunk and all it's potential
    /// sub chunks.
    pub fn to_vec(&self) -> Vec<Sample> {
        match self {
            Chunk::SubChunk { chunks, .. } => {
                let mut samples: Vec<Sample> = vec![];
                for chunk in chunks {
                    samples.extend(chunk.to_vec());
                }
                samples
            }
            Chunk::Leave { samples, .. } => samples.clone(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Chunk::SubChunk { metrics, .. } | Chunk::Leave { metrics, .. } => metrics.count,
        }
    }
}

// impl Default for Chunk {
//     fn default() -> Self {
//         let samples = vec![];
//         let metrics = SampleMetrics::default();

//         Chunk::Leave { samples, metrics }
//     }
// }

#[cfg(test)]
mod tests {
    use super::{Btree, Sample};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn btree_single_insertion() {
        let mut tree = Btree::default();

        // Insert some samples:
        let t1 = TimeStamp::new(1.0);
        let sample = Sample::new(t1, 3.1415926);
        tree.insert(sample);

        assert_eq!(tree.to_vec().len(), 1);

        // Check length:
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn btree_mutliple_insertions() {
        let mut tree = Btree::default();

        // Insert some samples:
        for i in 0..1000 {
            let t1 = TimeStamp::new(i as f64);
            let sample = Sample::new(t1, i as f64);
            tree.insert(sample);
        }

        // Check plain to vector:
        assert_eq!(tree.to_vec().len(), 1000);

        // Check length:
        // TODO:
        // assert_eq!(tree.len(), 1000);
    }
}
