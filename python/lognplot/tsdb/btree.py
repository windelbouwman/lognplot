""" B-tree data structure.

Idea is to create summary levels on top of chunks of data.
"""

import abc
from .metrics import Metrics, sample_to_metric, merge_metrics, samples_to_metric


class Btree:
    def __init__(self):
        self._leave_max = 32
        self._internal_node_max = 5
        self.root_node = BtreeLeaveNode(self._leave_max)

    @property
    def metrics(self):
        return self.root_node.metrics

    def append(self, sample):
        """ Append a single sample. """
        root_sibling = self.root_node.append(sample)
        if root_sibling:
            self._new_root(root_sibling)

    def extend(self, samples):
        """ Append a batch of samples. """
        # TODO: this can be improved for efficiency.
        bulk_fill = False
        if bulk_fill:
            raise NotImplementedError("bulk load")
        else:
            for sample in samples:
                self.append(sample)

    def _new_root(self, root_sibling):
        """ Construct a new root from the old root and a sibling. """
        # print("new root!")
        old_root = self.root_node
        self.root_node = BtreeInternalNode(self._internal_node_max)
        self.root_node.add_child(old_root)
        self.root_node.add_child(root_sibling)

    def __len__(self):
        # Return total number of samples!
        return self.metrics.count

    def __iter__(self):
        for sample in self.root_node:
            yield sample

    def query(self, selection_timespan, min_count):
        """ Query this tree for some data between the given points.
        """

        # Initial query result:
        nodes = self.root_node.select_range(selection_timespan)

        # Enhance resolution, while not enough samples.
        while nodes and len(nodes) < min_count and isinstance(nodes[0], BtreeNode):
            nodes = enhance(nodes, selection_timespan)

        # Take metrics from internal nodes:
        if nodes and isinstance(nodes[0], BtreeNode):
            nodes = [n.metrics for n in nodes]

        return nodes

    def query_metrics(self, selection_timespan):
        """ Retrieve metrics from a given range. """
        # TODO: refactor!
        nodes = self.query(selection_timespan, 500)
        if nodes:
            if isinstance(nodes[0], Metrics):
                metrics = merge_metrics(nodes)
            else:
                metrics = samples_to_metric(nodes)
            return metrics


def enhance(nodes, selection_span):
    """ Enhance resolution by descending into child nodes in the selected time span.
    """
    assert nodes
    new_nodes = []
    if len(nodes) == 1:
        new_nodes.extend(nodes[0].select_range(selection_span))
    else:
        assert len(nodes) > 1
        new_nodes.extend(nodes[0].select_range(selection_span))
        for node in nodes[1:-1]:
            new_nodes.extend(node.select_all())
        new_nodes.extend(nodes[-1].select_range(selection_span))
    return new_nodes


def overlap(span1, span2):
    """ Test if two spans overlap.

    Parameters are two spans, which are tuples of (begin, end).
    """
    assert span1[0] <= span1[1]
    assert span2[0] <= span2[1]

    return span1[0] <= span2[1] and span2[0] <= span1[1]


class BtreeNode(metaclass=abc.ABCMeta):
    """ Base class for either internal, or leave nodes of the B-tree.

    This class and it's subclasses are for internal usage in the B-tree.
    Do not use outside this file.
    """

    def __init__(self):
        pass

    @abc.abstractmethod
    def append(self, sample):
        raise NotImplementedError()

    @abc.abstractmethod
    def append_leave(self, leave_node):
        raise NotImplementedError()

    @abc.abstractmethod
    def select_range(self, selection_span):
        raise NotImplementedError()

    @abc.abstractmethod
    def select_all(self):
        raise NotImplementedError()


class BtreeInternalNode(BtreeNode):
    """ Intermediate level node in the B-tree.
    
    Has child nodes of either internal node, or leave type.
    """

    def __init__(self, max_children):
        self._children = []
        self.max_children = max_children
        self._metrics = None

    @property
    def metrics(self):
        if self._metrics is None:
            return self.calculate_metrics_from_children()
        else:
            assert self._metrics
            return self._metrics

    def calculate_metrics_from_children(self):
        # print(self._children)
        child_metrics = [c.metrics for c in self._children]
        # print(child_metrics)
        return merge_metrics(child_metrics)

    def add_child(self, child_node):
        self._children.append(child_node)

    def append(self, sample):
        """ Append a single sample to a descendant of this node.
        """
        last_child = self._children[-1]
        new_child = last_child.append(sample)
        if new_child:
            return self.append_child(new_child)
        else:
            pass  # We are Ok

    def append_leave(self, leave_node):
        """ Append a leave node. """
        last_child = self._children[-1]
        new_child = last_child.append_leave(leave_node)
        if new_child:
            return self.append_child(new_child)

    def append_child(self, child_node):
        if len(self._children) < self.max_children:
            self.add_child(child_node)
        else:
            # We are full, calculate metrics!
            self._metrics = self.calculate_metrics_from_children()
            new_sibling = BtreeInternalNode(self.max_children)
            new_sibling.add_child(child_node)
            return new_sibling

    def __iter__(self):
        # TBD: At this moment, we iterate over all samples
        # recursing into child nodes. This might be counter-intuitive.
        for child in self._children:
            for sample in child:
                yield sample

    def select_range(self, selection_span):
        """ Select a range of nodes falling between `begin` and `end` """
        assert self._children

        in_range_children = []
        full_span = (self.metrics.x1, self.metrics.x2)
        if overlap(full_span, selection_span):
            # In range, so:
            # Some overlap!
            # Find first node:
            for node in self._children:
                node_span = (node.metrics.x1, node.metrics.x2)
                if overlap(selection_span, node_span):
                    in_range_children.append(node)
        else:
            # out of range
            pass

        return in_range_children

    def select_all(self):
        return self._children


class BtreeLeaveNode(BtreeNode):
    """ A leave node in the B-tree.
    
    This node type actually contains raw observations.
    """

    def __init__(self, max_samples):
        self.samples = []
        self.max_samples = max_samples
        self._metrics = None

    @property
    def metrics(self):
        return self._metrics

    @property
    def full(self):
        return len(self.samples) >= self.max_samples

    def _add_sample(self, sample):
        self.samples.append(sample)

        # Update metrics:
        metric = sample_to_metric(sample)
        if self._metrics:
            self._metrics = self._metrics + metric
        else:
            self._metrics = metric

    def append(self, sample):
        if len(self.samples) < self.max_samples:
            self._add_sample(sample)
        else:
            # print('new leave sibling!')
            new_sibling = BtreeLeaveNode(self.max_samples)
            new_sibling._add_sample(sample)
            return new_sibling

    def append_leave(self, leave_node):
        assert self.full
        return leave_node

    def __iter__(self):
        for sample in self.samples:
            yield sample

    def select_range(self, selection_span):
        """ Select a range of samples falling between `begin` and `end` """
        if not self.samples:
            return []

        in_range_samples = []
        full_span = (self.metrics.x1, self.metrics.x2)
        if overlap(full_span, selection_span):
            # In range, so:
            # Some overlap!
            # Find first node:
            for sample in self.samples:
                if selection_span[0] <= sample[0] <= selection_span[1]:
                    in_range_samples.append(sample)
        else:
            # out of range
            pass

        return in_range_samples

    def select_all(self):
        """ Retrieve all samples in this node.
        """
        return self.samples
