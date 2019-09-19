use super::metrics::SampleMetrics;
use super::sample::Sample;

const CHUNK_SIZE: usize = 256;

pub enum Chunk {
    SubChunk {
        chunks: Vec<Chunk>,
    },

    /// A leave chunk with some samples in it.
    Leave {
        samples: Vec<Sample>,
        metric: SampleMetrics,
    },
}

impl Chunk {
    pub fn push(&mut self, value: Sample) {
        match self {
            Chunk::SubChunk { .. } => {}
            Chunk::Leave { samples, metric } => {
                metric.update(&value);
                samples.push(value);
            }
        }
    }

    pub fn to_vec(&self) -> Vec<Sample> {
        match self {
            Chunk::SubChunk { .. } => {
                unimplemented!("TODO");
            }
            Chunk::Leave { samples, .. } => samples.clone(),
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        let samples = vec![];
        let metric = SampleMetrics::default();

        Chunk::Leave { samples, metric }
    }
}
