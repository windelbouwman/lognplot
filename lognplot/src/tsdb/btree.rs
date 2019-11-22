//! Implementation of a B-tree like data structure.
//!
//! The idea is to create leave nodes and intermediate nodes.
//! Leave and intermediate nodes can have multiple child nodes.

use super::aggregation::Aggregation;
use super::metrics::{Metrics, SampleMetrics};
use super::sample::Sample;
use crate::time::TimeSpan;

use std::cell::RefCell;

/// This implements a b-tree structure.
#[derive(Debug)]
pub struct Btree {
    root: RefCell<Node<SampleMetrics>>,
}

/// Create an empty b-tree
impl Default for Btree {
    fn default() -> Self {
        let root = RefCell::new(Node::new_leave());
        Btree { root }
    }
}

impl Btree {
    /// Append a sample to the tree
    pub fn append_sample(&self, sample: Sample) {
        // Strategy, traverse down, until a leave, and split on the way back upwards if
        // required.
        // Find proper chunk, or create one if required.
        let optionally_root_split = self.root.borrow_mut().append_sample(sample);

        if let Some(root_sibling) = optionally_root_split {
            let new_root = Node::new_intermediate();
            let old_root = self.root.replace(new_root);
            self.root.borrow_mut().add_child(old_root);
            self.root.borrow_mut().add_child(root_sibling);
        }
    }

    /// Bulk import samples.
    pub fn append_samples(&self, samples: Vec<Sample>) {
        for sample in samples {
            self.append_sample(sample);
        }
    }

    /// Query the tree for some data.
    pub fn query_range(&self, timespan: &TimeSpan, min_items: usize) -> Vec<Sample> {
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
enum Node<M>
where
    M: Metrics,
{
    /// An intermediate chunk with some leave
    /// chunk.
    SubChunk(InternalNode<M>),

    /// A leave chunk with some samples in it.
    Leave(LeaveNode<M>),
    // TODO: in future support on disk node?
}

/// Intermediate node
#[derive(Debug)]
struct InternalNode<M>
where
    M: Metrics,
{
    children: Vec<Node<M>>,
    metrics: Option<Aggregation<M>>,
}

/// Leave node type
#[derive(Debug)]
struct LeaveNode<M>
where
    M: Metrics,
{
    samples: Vec<Sample>,
    metrics: Option<Aggregation<M>>,
}

impl<M> Node<M>
where
    M: Metrics + Clone,
{
    fn new_intermediate() -> Self {
        Node::SubChunk(InternalNode::new())
    }

    fn new_leave() -> Self {
        Node::Leave(LeaveNode::new())
    }

    /// Test if this chunk is full
    fn is_full(&self) -> bool {
        match self {
            Node::Leave(leave) => leave.is_full(),
            Node::SubChunk(internal) => internal.is_full(),
        }
    }

    fn add_child(&mut self, child: Node<M>) {
        match self {
            Node::SubChunk(internal_node) => internal_node.add_child(child),
            x => panic!("Wrong node type to add a child to"),
        }
    }

    /// The append sample to operation!
    fn append_sample(&mut self, sample: Sample) -> Option<Node<M>> {
        match self {
            Node::SubChunk(internal_node) => {
                internal_node.append_sample(sample).map(Node::SubChunk)
            }
            Node::Leave(leave_node) => leave_node.append_sample(sample).map(Node::Leave),
        }
    }

    /// Get all samples from this chunk and all it's potential
    /// sub chunks.
    fn to_vec(&self) -> Vec<Sample> {
        match self {
            Node::SubChunk(internal) => internal.to_vec(),
            Node::Leave(leave) => leave.to_vec(),
        }
    }

    /// Get metrics for this node
    fn metrics(&self) -> Option<Aggregation<M>> {
        match self {
            Node::Leave(leave) => leave.metrics(),
            Node::SubChunk(internal) => internal.metrics(),
        }
    }

    fn len(&self) -> usize {
        self.metrics().map_or(0, |m| m.count)
    }
}

impl<M> InternalNode<M>
where
    M: Metrics + Clone,
{
    fn new() -> Self {
        InternalNode {
            children: Vec::with_capacity(CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    fn is_full(&self) -> bool {
        self.children.len() >= CHUNK_SIZE
    }

    fn metrics(&self) -> Option<Aggregation<M>> {
        if self.metrics.is_some() {
            // We have pre-calculated metrics!
            self.metrics.clone()
        } else {
            self.calculate_metrics_from_child_nodes()
        }
    }

    fn calculate_metrics_from_child_nodes(&self) -> Option<Aggregation<M>> {
        let mut metrics: Option<Aggregation<M>> = None;
        for child in &self.children {
            if let Some(child_metrics) = child.metrics() {
                if let Some(metrics2) = &mut metrics {
                    metrics2.include(&child_metrics);
                } else {
                    metrics = Some(child_metrics);
                }
            }
        }
        metrics
    }

    fn append_sample(&mut self, sample: Sample) -> Option<InternalNode<M>> {
        // For now alway insert into last chunk:
        let optional_new_chunk = self.children.last_mut().unwrap().append_sample(sample);

        // Optionally we have a new chunk which must be added.
        if let Some(new_child) = optional_new_chunk {
            if self.children.len() < CHUNK_SIZE {
                self.add_child(new_child);
                None
            } else {
                self.metrics = self.calculate_metrics_from_child_nodes();
                // Split required!
                // for now, just split by creating a new node.
                //  debug!("Split of sub chunk node");
                let mut new_sibling = InternalNode::new();
                new_sibling.add_child(new_child);
                Some(new_sibling)
            }
        } else {
            None
        }
    }

    /// Append a chunk into this chunk.
    /// Note: chunk must be of variant subchunk, otherwise this
    /// will fail.
    fn add_child(&mut self, child: Node<M>) {
        assert!(self.children.len() < CHUNK_SIZE);
        self.children.push(child);
    }

    fn to_vec(&self) -> Vec<Sample> {
        let mut samples: Vec<Sample> = vec![];
        for child in &self.children {
            samples.extend(child.to_vec());
        }
        samples
    }
}

impl<M> LeaveNode<M>
where
    M: Metrics + Clone,
{
    /// Create a new leave chunk!
    fn new() -> Self {
        LeaveNode {
            samples: Vec::with_capacity(CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    fn is_full(&self) -> bool {
        self.samples.len() >= CHUNK_SIZE
    }

    fn metrics(&self) -> Option<Aggregation<M>> {
        self.metrics.clone()
    }

    fn append_sample(&mut self, sample: Sample) -> Option<LeaveNode<M>> {
        if self.samples.len() < CHUNK_SIZE {
            self.add_sample(sample);
            None
        } else {
            // We must split!
            // debug!("Split of leave node!");
            let mut new_leave = LeaveNode::new();
            new_leave.add_sample(sample);
            Some(new_leave)
        }
    }

    fn add_sample(&mut self, sample: Sample) {
        assert!(self.samples.len() < CHUNK_SIZE);

        if self.metrics.is_none() {
            self.metrics = Some(Aggregation::from_sample(&sample))
        } else {
            self.metrics.as_mut().unwrap().update(&sample);
        }
        self.samples.push(sample);
    }

    fn to_vec(&self) -> Vec<Sample> {
        self.samples.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Btree, Sample};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn btree_single_insertion() {
        let tree = Btree::default();

        // Insert some samples:
        let t1 = TimeStamp::from_seconds(1);
        let sample = Sample::new(t1, 3.1415926);
        tree.append_sample(sample);

        assert_eq!(tree.to_vec().len(), 1);

        // Check length:
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn btree_mutliple_insertions() {
        let tree = Btree::default();

        // Insert some samples:
        for i in 0..1000 {
            let t1 = TimeStamp::from_seconds(i);
            let sample = Sample::new(t1, i as f64);
            tree.append_sample(sample);
        }

        // Check plain to vector:
        assert_eq!(tree.to_vec().len(), 1000);

        // Check length:
        assert_eq!(tree.len(), 1000);

        // Check query
        let time_span = TimeSpan::new(TimeStamp::from_seconds(3), TimeStamp::from_seconds(13));
        tree.query_range(&time_span, 9);
    }
}
