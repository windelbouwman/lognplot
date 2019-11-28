//! Implementation of a B-tree like data structure.
//!
//! The idea is to create leave nodes and intermediate nodes.
//! Leave and intermediate nodes can have multiple child nodes.

use super::metrics::Metrics;
use super::{Aggregation, Observation};
use crate::time::TimeSpan;

use std::cell::RefCell;

/// This is the intermediate level fanout ratio.
/// A higher number yields less overhead (zoom levels)
const INTERMEDIATE_CHUNK_SIZE: usize = 5;

/// This constant defines the fanout ratio.
/// Each leave contains maximum this number of values
/// Also, each intermediate node also contains this maximum number
/// of subchunks.
const LEAVE_CHUNK_SIZE: usize = 16;

/// This implements a b-tree structure.
///
/// The tree structure supports fast lookup
/// of time ranges.
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

        let optionally_root_split = self.root.borrow_mut().append_observation(observation);

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
    ///
    /// This will go into deeper levels of detail, until a certain
    /// amount of data points is found.
    pub fn query_range(&self, timespan: &TimeSpan, min_items: usize) -> RangeQueryResult<V, M> {
        let root_node = self.root.borrow();
        let mut selection = root_node.select_range(timespan);

        while (selection.len() < min_items) && selection.can_enhance() {
            selection = selection.enhance(timespan);
        }

        selection.into_query_result()
    }

    /// Get a data summary about the given time span.
    ///
    /// Strategy here is to go into child nodes at the
    /// edges of the selection. The middle child nodes
    /// can be aggregated earlier on.
    pub fn range_summary(&self, timespan: &TimeSpan) -> Option<Aggregation<V, M>> {
        // Start with a selection in the root node
        let root_node = self.root.borrow();
        let mut partially_selected_nodes: Vec<&Node<V, M>> = vec![&root_node];
        let mut selected_nodes = vec![];
        let mut selected_observations: Vec<Observation<V>> = vec![];
        // let mut selection = root_node.select_range(timespan);

        while !partially_selected_nodes.is_empty() {
            let partial_node = partially_selected_nodes.pop().unwrap();
            let selection = partial_node.select_range(timespan);
            match selection {
                RangeSelectionResult::Nodes(nodes) => {
                    for node in nodes {
                        if let Some(aggregation) = node.metrics() {
                            if timespan.covers(&aggregation.timespan) {
                                selected_nodes.push(aggregation);
                            } else {
                                partially_selected_nodes.push(node);
                            }
                        }
                    }
                }
                RangeSelectionResult::Observations(observations) => {
                    for observation in observations {
                        selected_observations.push(observation.clone());
                    }
                    // selected_observations.extend(observations.iter().map(|o| o.clone()).collect());
                }
            };
        }

        println!(
            "Nodes: {:?}, observations: {:?}",
            selected_nodes.len(),
            selected_observations.len()
        );

        // Now we have nodes and individual observations, take metrics of those.
        let all_aggregations: Vec<Option<Aggregation<V, M>>> = vec![
            Aggregation::from_aggregations(&selected_nodes),
            Aggregation::from_observations(&selected_observations),
        ];
        let all_aggregations: Vec<Aggregation<V, M>> =
            all_aggregations.into_iter().filter_map(|a| a).collect();
        let summary = Aggregation::from_aggregations(&all_aggregations);

        // assert!(timespan.covers(summary.timespan));

        summary
    }

    /// Get a summary about all data in this tree.
    pub fn summary(&self) -> Option<Aggregation<V, M>> {
        self.root.borrow().metrics()
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

impl<V, M> RangeQueryResult<V, M>
where
    M: Metrics<V> + From<V>,
{
    pub fn len(&self) -> usize {
        match self {
            RangeQueryResult::Observations(observations) => observations.len(),
            RangeQueryResult::Aggregations(aggregations) => aggregations.len(),
        }
    }
}

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
    Intermediate(InternalNode<V, M>),

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
    observations: Vec<Observation<V>>,
    metrics: Option<Aggregation<V, M>>,
}

impl<V, M> Node<V, M>
where
    M: Metrics<V> + Clone + From<V>,
    V: Clone,
{
    fn new_intermediate() -> Self {
        Node::Intermediate(InternalNode::new())
    }

    fn new_leave() -> Self {
        Node::Leave(LeaveNode::new())
    }

    /// Test if this chunk is full
    fn _is_full(&self) -> bool {
        match self {
            Node::Leave(leave) => leave.is_full(),
            Node::Intermediate(internal) => internal.is_full(),
        }
    }

    fn add_child(&mut self, child: Node<V, M>) {
        match self {
            Node::Intermediate(internal_node) => internal_node.add_child(child),
            _x => panic!("Wrong node type to add a child to"),
        }
    }

    /// The append observation to operation!
    fn append_observation(&mut self, observation: Observation<V>) -> Option<Node<V, M>> {
        match self {
            Node::Intermediate(internal_node) => internal_node
                .append_observation(observation)
                .map(Node::Intermediate),
            Node::Leave(leave_node) => leave_node.append_observation(observation).map(Node::Leave),
        }
    }

    /// Select all child elements
    fn select_all(&self) -> RangeSelectionResult<V, M> {
        match self {
            Node::Intermediate(internal) => RangeSelectionResult::Nodes(internal.select_all()),
            Node::Leave(leave) => RangeSelectionResult::Observations(leave.select_all()),
        }
    }

    /// Select a timespan of elements
    fn select_range(&self, timespan: &TimeSpan) -> RangeSelectionResult<V, M> {
        match self {
            Node::Intermediate(internal) => {
                RangeSelectionResult::Nodes(internal.select_range(timespan))
            }
            Node::Leave(leave) => RangeSelectionResult::Observations(leave.select_range(timespan)),
        }
    }

    /// Get all samples from this chunk and all it's potential
    /// sub chunks.
    fn to_vec(&self) -> Vec<Observation<V>> {
        match self {
            Node::Intermediate(internal) => internal.to_vec(),
            Node::Leave(leave) => leave.to_vec(),
        }
    }

    /// Get metrics for this node
    fn metrics(&self) -> Option<Aggregation<V, M>> {
        match self {
            Node::Leave(leave) => leave.metrics(),
            Node::Intermediate(internal) => internal.metrics(),
        }
    }

    fn len(&self) -> usize {
        self.metrics().map_or(0, |m| m.count)
    }
}

/// The result of selecting a time range on a node.
enum RangeSelectionResult<'t, V, M>
where
    M: Metrics<V> + From<V>,
{
    Nodes(Vec<&'t Node<V, M>>),
    Observations(Vec<&'t Observation<V>>),
}

impl<'t, V, M> RangeSelectionResult<'t, V, M>
where
    M: Metrics<V> + From<V> + Clone,
    V: Clone,
{
    fn len(&self) -> usize {
        match self {
            RangeSelectionResult::Nodes(nodes) => nodes.len(),
            RangeSelectionResult::Observations(observations) => observations.len(),
        }
    }

    // fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }

    /// Test if we can enhance this selection result any further.
    fn can_enhance(&self) -> bool {
        match self {
            RangeSelectionResult::Nodes(nodes) => !nodes.is_empty(),
            RangeSelectionResult::Observations(_) => false,
        }
    }

    /// Zoom in on a sequence of nodes, by selecting the
    /// child nodes which are in range.
    fn enhance(self, timespan: &TimeSpan) -> RangeSelectionResult<'t, V, M> {
        match self {
            RangeSelectionResult::Nodes(nodes) => {
                assert!(!nodes.is_empty());

                if nodes.len() == 1 {
                    // Special case of a single node.
                    nodes.first().unwrap().select_range(timespan)
                } else {
                    // perform select range on first and last node, select all on the middle nodes.
                    assert!(nodes.len() > 1);
                    let (first, tail) = nodes.split_first().unwrap();
                    let (last, middle) = tail.split_last().unwrap();

                    let mut result = first.select_range(timespan); // first
                    for node in middle {
                        result.extend(node.select_all()); // middle
                    }
                    result.extend(last.select_range(timespan)); // last

                    result
                }
            }
            RangeSelectionResult::Observations(_) => {
                panic!("This should never happen. Do not call enhance on observation level.")
            }
        }
    }

    /// Append a second selection to this selection!
    fn extend(&mut self, mut other: Self) {
        match self {
            RangeSelectionResult::Observations(observations) => {
                if let RangeSelectionResult::Observations(other_observations) = &mut other {
                    observations.append(other_observations);
                } else {
                    panic!("Can only concatenate selection results of the same type");
                }
            }
            RangeSelectionResult::Nodes(nodes) => {
                if let RangeSelectionResult::Nodes(other_nodes) = &mut other {
                    nodes.append(other_nodes)
                } else {
                    panic!("Can only concatenate selection results of the same type");
                }
            }
        }
    }

    /// Convert selection into query result!
    fn into_query_result(self) -> RangeQueryResult<V, M> {
        match self {
            RangeSelectionResult::Nodes(nodes) => RangeQueryResult::Aggregations(
                nodes.into_iter().map(|n| n.metrics().unwrap()).collect(),
            ),
            RangeSelectionResult::Observations(observations) => {
                RangeQueryResult::Observations(observations.into_iter().cloned().collect())
            }
        }
    }
}

impl<V, M> InternalNode<V, M>
where
    M: Metrics<V> + Clone + From<V>,
    V: Clone,
{
    fn new() -> Self {
        InternalNode {
            children: Vec::with_capacity(INTERMEDIATE_CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    fn is_full(&self) -> bool {
        self.children.len() >= INTERMEDIATE_CHUNK_SIZE
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
                    metrics2.include_aggregation(&child_metrics);
                } else {
                    metrics = Some(child_metrics);
                }
            }
        }
        metrics
    }

    fn append_observation(&mut self, observation: Observation<V>) -> Option<InternalNode<V, M>> {
        // For now alway insert into last chunk:
        let optional_new_chunk = self
            .children
            .last_mut()
            .unwrap()
            .append_observation(observation);

        // Optionally we have a new chunk which must be added.
        if let Some(new_child) = optional_new_chunk {
            if self.is_full() {
                self.metrics = self.calculate_metrics_from_child_nodes();
                // Split required!
                // for now, just split by creating a new node.
                //  debug!("Split of sub chunk node");
                let mut new_sibling = InternalNode::new();
                new_sibling.add_child(new_child);
                Some(new_sibling)
            } else {
                self.add_child(new_child);
                None
            }
        } else {
            None
        }
    }

    /// Append a chunk into this chunk.
    /// Note: chunk must be of variant subchunk, otherwise this
    /// will fail.
    fn add_child(&mut self, child: Node<V, M>) {
        assert!(!self.is_full());
        self.children.push(child);
    }

    /// Select child nodes in range.
    fn select_range(&self, timespan: &TimeSpan) -> Vec<&Node<V, M>> {
        let mut in_range_nodes = vec![];

        for child in &self.children {
            if let Some(child_metrics) = child.metrics() {
                if child_metrics.timespan.overlap(timespan) {
                    in_range_nodes.push(child);
                }
            }
        }

        in_range_nodes
    }

    /// Select all child nodes.
    fn select_all(&self) -> Vec<&Node<V, M>> {
        self.children.iter().collect()
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
            observations: Vec::with_capacity(LEAVE_CHUNK_SIZE),
            metrics: Default::default(),
        }
    }

    /// Test if this leave is full or not.
    fn is_full(&self) -> bool {
        self.observations.len() >= LEAVE_CHUNK_SIZE
    }

    fn metrics(&self) -> Option<Aggregation<V, M>> {
        self.metrics.clone()
    }

    /// Append a single observation to this tree.
    /// If the node is full, return a new leave node.
    fn append_observation(&mut self, observation: Observation<V>) -> Option<LeaveNode<V, M>> {
        if self.is_full() {
            // We must split!
            // debug!("Split of leave node!");
            let mut new_leave = LeaveNode::new();
            new_leave.add_sample(observation);
            Some(new_leave)
        } else {
            self.add_sample(observation);
            None
        }
    }

    fn add_sample(&mut self, observation: Observation<V>) {
        assert!(!self.is_full());

        // Update metrics:
        if self.metrics.is_none() {
            self.metrics = Some(Aggregation::from(observation.clone()))
        } else {
            self.metrics
                .as_mut()
                .unwrap()
                .include_observation(&observation);
        }

        self.observations.push(observation);
    }

    /// Select the observations from this leave which fall into the given
    /// timespan.
    fn select_range(&self, timespan: &TimeSpan) -> Vec<&Observation<V>> {
        let mut in_range_observations = vec![];

        for observation in &self.observations {
            if timespan.contains(&observation.timestamp) {
                in_range_observations.push(observation);
            }
        }

        in_range_observations
    }

    fn select_all(&self) -> Vec<&Observation<V>> {
        self.observations.iter().collect()
    }

    fn to_vec(&self) -> Vec<Observation<V>> {
        self.observations.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::super::sample::{Sample, SampleMetrics};
    use super::{Btree, Observation};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn btree_single_insertion() {
        let tree = Btree::<Sample, SampleMetrics>::default();

        // Insert some samples:
        let t1 = TimeStamp::from_seconds(1);
        let sample1 = Sample::new(3.1415926);
        let observation = Observation::new(t1, sample1);
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
            let sample = Sample::new(i as f64);
            let observation = Observation::new(t1, sample);
            tree.append_sample(observation);
        }

        // Check plain to vector:
        assert_eq!(tree.to_vec().len(), 1000);

        // Check length:
        assert_eq!(tree.len(), 1000);

        // Check query
        let time_span = TimeSpan::from_seconds(3, 13);
        let result = tree.query_range(&time_span, 9);
        assert_eq!(result.len(), 11);
    }
}
