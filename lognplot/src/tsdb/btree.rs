//! Implementation of a B-tree like data structure.
//!
//! The idea is to create leave nodes and intermediate nodes.
//! Leave and intermediate nodes can have multiple child nodes.

use super::metrics::Metrics;
use super::{Aggregation, Observation};
use crate::time::TimeSpan;

use std::cell::RefCell;

/// This implements a b-tree structure.
#[derive(Debug)]
pub struct Btree<V, M>
where
    M: Metrics<V> + From<V>,
{
    root: RefCell<Node<V, M>>,
}

/// Create an empty b-tree
impl<V, M> Default for Btree<V, M>
where
    M: Metrics<V> + From<V> + Clone,
    V: Clone,
{
    fn default() -> Self {
        let root = RefCell::new(Node::new_leave());
        Btree { root }
    }
}

impl<V, M> Btree<V, M>
where
    M: Metrics<V> + From<V> + Clone,
    V: Clone,
{
    /// Append a sample to the tree
    pub fn append_sample(&self, observation: Observation<V>) {
        // Strategy, traverse down, until a leave, and split on the way back upwards if
        // required.
        // Find proper chunk, or create one if required.

        let optionally_root_split = self.root.borrow_mut().append_sample(observation);

        if let Some(root_sibling) = optionally_root_split {
            let new_root = Node::new_intermediate();
            let old_root = self.root.replace(new_root);
            self.root.borrow_mut().add_child(old_root);
            self.root.borrow_mut().add_child(root_sibling);
        }
    }

    /// Bulk import samples.
    // pub fn append_samples(&self, samples: Vec<Sample>) {
    //     for sample in samples {
    //         self.append_sample(sample);
    //     }
    // }

    /// Query the tree for some data.
    pub fn query_range(&self, timespan: &TimeSpan, min_items: usize) -> RangeQueryResult<V, M> {
        // big TODO
        let samples = vec![];
        RangeQueryResult::Observations(samples)
    }

    /// Get a flat list of all observation in this tree.
    pub fn to_vec(&self) -> Vec<Observation<V>> {
        self.root.borrow().to_vec()
    }

    pub fn len(&self) -> usize {
        self.root.borrow().len()
    }
}

/// Inner results, can be either a series of single
/// observations, or a series of aggregate observations.
#[derive(Debug)]
pub enum RangeQueryResult<V, M>
where
    M: Metrics<V> + From<V>,
{
    Observations(Vec<Observation<V>>),
    Aggregations(Vec<Aggregation<V, M>>),
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
enum Node<V, M>
where
    M: Metrics<V> + From<V>,
{
    /// An intermediate chunk with some leave
    /// chunk.
    SubChunk(InternalNode<V, M>),

    /// A leave chunk with some samples in it.
    Leave(LeaveNode<V, M>),
    // TODO: in future support on disk node?
}

impl<V, M> Default for Node<V, M>
where
    M: Metrics<V> + Clone + From<V>,
    V: Clone,
{
    fn default() -> Self {
        Node::new_leave()
    }
}

/// Intermediate node
#[derive(Debug)]
struct InternalNode<V, M>
where
    M: Metrics<V> + From<V>,
{
    children: Vec<Node<V, M>>,
    metrics: Option<Aggregation<V, M>>,
}

/// Leave node type
#[derive(Debug)]
struct LeaveNode<V, M>
where
    M: Metrics<V> + From<V>,
{
    samples: Vec<Observation<V>>,
    metrics: Option<Aggregation<V, M>>,
}

impl<V, M> Node<V, M>
where
    M: Metrics<V> + Clone + From<V>,
    V: Clone,
{
    fn new_intermediate() -> Self {
        Node::SubChunk(InternalNode::new())
    }

    fn new_leave() -> Self {
        Node::Leave(LeaveNode::new())
    }

    /// Test if this chunk is full
    fn _is_full(&self) -> bool {
        match self {
            Node::Leave(leave) => leave._is_full(),
            Node::SubChunk(internal) => internal._is_full(),
        }
    }

    fn add_child(&mut self, child: Node<V, M>) {
        match self {
            Node::SubChunk(internal_node) => internal_node.add_child(child),
            _x => panic!("Wrong node type to add a child to"),
        }
    }

    /// The append sample to operation!
    fn append_sample(&mut self, sample: Observation<V>) -> Option<Node<V, M>> {
        match self {
            Node::SubChunk(internal_node) => {
                internal_node.append_sample(sample).map(Node::SubChunk)
            }
            Node::Leave(leave_node) => leave_node.append_sample(sample).map(Node::Leave),
        }
    }

    /// Get all samples from this chunk and all it's potential
    /// sub chunks.
    fn to_vec(&self) -> Vec<Observation<V>> {
        match self {
            Node::SubChunk(internal) => internal.to_vec(),
            Node::Leave(leave) => leave.to_vec(),
        }
    }

    /// Get metrics for this node
    fn metrics(&self) -> Option<Aggregation<V, M>> {
        match self {
            Node::Leave(leave) => leave.metrics(),
            Node::SubChunk(internal) => internal.metrics(),
        }
    }

    fn len(&self) -> usize {
        self.metrics().map_or(0, |m| m.count)
    }
}

impl<V, M> InternalNode<V, M>
where
    M: Metrics<V> + Clone + From<V>,
    V: Clone,
{
    fn new() -> Self {
        InternalNode {
            children: Vec::with_capacity(CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    fn _is_full(&self) -> bool {
        self.children.len() >= CHUNK_SIZE
    }

    fn metrics(&self) -> Option<Aggregation<V, M>> {
        if self.metrics.is_some() {
            // We have pre-calculated metrics!
            self.metrics.clone()
        } else {
            self.calculate_metrics_from_child_nodes()
        }
    }

    fn calculate_metrics_from_child_nodes(&self) -> Option<Aggregation<V, M>> {
        let mut metrics: Option<Aggregation<V, M>> = None;
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

    fn append_sample(&mut self, sample: Observation<V>) -> Option<InternalNode<V, M>> {
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
    fn add_child(&mut self, child: Node<V, M>) {
        assert!(self.children.len() < CHUNK_SIZE);
        self.children.push(child);
    }

    fn to_vec(&self) -> Vec<Observation<V>> {
        let mut samples: Vec<Observation<V>> = vec![];
        for child in &self.children {
            samples.extend(child.to_vec());
        }
        samples
    }
}

impl<V, M> LeaveNode<V, M>
where
    M: Metrics<V> + Clone + From<V>,
    V: Clone,
{
    /// Create a new leave chunk!
    fn new() -> Self {
        LeaveNode {
            samples: Vec::with_capacity(CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    fn _is_full(&self) -> bool {
        self.samples.len() >= CHUNK_SIZE
    }

    fn metrics(&self) -> Option<Aggregation<V, M>> {
        self.metrics.clone()
    }

    /// Append a single observation to this tree.
    /// If the node is full, return a new leave node.
    fn append_sample(&mut self, sample: Observation<V>) -> Option<LeaveNode<V, M>> {
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

    fn add_sample(&mut self, sample: Observation<V>) {
        assert!(self.samples.len() < CHUNK_SIZE);

        if self.metrics.is_none() {
            self.metrics = Some(Aggregation::from(sample.clone()))
        } else {
            self.metrics.as_mut().unwrap().update(&sample);
        }
        self.samples.push(sample);
    }

    fn to_vec(&self) -> Vec<Observation<V>> {
        self.samples.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::super::metrics::SampleMetrics;
    use super::super::Sample;
    use super::{Btree, Observation};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn btree_single_insertion() {
        let tree = Btree::<Sample, SampleMetrics>::default();

        // Insert some samples:
        let t1 = TimeStamp::from_seconds(1);
        let sample1 = Sample::new(t1.clone(), 3.1415926);
        let observation = Observation::new(t1.clone(), sample1);
        tree.append_sample(observation);

        assert_eq!(tree.to_vec().len(), 1);

        // Check length:
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn btree_mutliple_insertions() {
        let tree = Btree::<Sample, SampleMetrics>::default();

        // Insert some samples:
        for i in 0..1000 {
            let t1 = TimeStamp::from_seconds(i);
            let sample = Sample::new(t1.clone(), i as f64);
            let observation = Observation::new(t1, sample);
            tree.append_sample(observation);
        }

        // Check plain to vector:
        assert_eq!(tree.to_vec().len(), 1000);

        // Check length:
        assert_eq!(tree.len(), 1000);

        // Check query
        let time_span = TimeSpan::new(TimeStamp::from_seconds(3), TimeStamp::from_seconds(13));
        let _result = tree.query_range(&time_span, 9);
        // TODO: check result.
    }
}
