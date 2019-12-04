""" B-tree data structure.

Idea is to create summary levels on top of chunks of data.
"""

import abc
from .metrics import Metrics
from .aggregation import Aggregation
from ..time import TimeSpan


class Btree:
    def __init__(self):
        self._leave_max = 32
        self._internal_node_max = 5
        self.root_node = BtreeLeaveNode(self._leave_max)

    @property
    def aggregation(self):
        return self.root_node.aggregation

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
        return self.aggregation.metrics.count

    def __iter__(self):
        for sample in self.root_node:
            yield sample

    def query(self, selection_timespan: TimeSpan, min_count):
        """ Query this tree for some data between the given points.
        """

        # Initial query result:
        selection = self.root_node.select_range(selection_timespan)

        # Enhance resolution, while not enough samples.
        while (
            selection
            and len(selection) < min_count
            and isinstance(selection[0], BtreeNode)
        ):
            selection = enhance(selection, selection_timespan)

        # Take metrics from internal nodes:
        if selection and isinstance(selection[0], BtreeNode):
            selection = [n.aggregation for n in selection]

        return selection

    def query_metrics(self, selection_timespan: TimeSpan) -> Aggregation:
        """ Retrieve aggregation from a given range. """

        partially_selected = [self.root_node]
        selected_aggregations = []
        selected_samples = []

        while partially_selected:
            partial_node = partially_selected.pop()
            selection = partial_node.select_range(selection_timespan)
            if selection:
                if isinstance(selection[0], BtreeNode):
                    for node in selection:
                        aggregation = node.aggregation
                        if selection_timespan.covers(aggregation.timespan):
                            selected_aggregations.append(aggregation)
                        else:
                            partially_selected.append(node)
                else:
                    selected_samples.extend(selection)

        # print(len(selected_aggregations), len(selected_samples))

        if selected_samples:
            selected_aggregations.append(Aggregation.from_samples(selected_samples))

        if selected_aggregations:
            return Aggregation.from_aggregations(selected_aggregations)

    def last_value(self):
        """ Get last item in the collection """
        return self.root_node.last_value()


def enhance(nodes, selection_span):
    """ Enhance resolution by descending into child nodes in the selected time span.
    """
    assert nodes
    new_nodes = []
    if len(nodes) == 1:
        new_nodes.extend(nodes[0].select_range(selection_span))
    else:
        # Assume here first and last selected node overlap partially.
        assert len(nodes) > 1
        new_nodes.extend(nodes[0].select_range(selection_span))
        for node in nodes[1:-1]:
            new_nodes.extend(node.select_all())
        new_nodes.extend(nodes[-1].select_range(selection_span))
    return new_nodes


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
    def select_range(self, selection_span: TimeSpan):
        raise NotImplementedError()

    @abc.abstractmethod
    def select_all(self):
        raise NotImplementedError()

    @abc.abstractmethod
    def last_value(self):
        raise NotImplementedError()


class BtreeInternalNode(BtreeNode):
    """ Intermediate level node in the B-tree.
    
    Has child nodes of either internal node, or leave type.
    """

    def __init__(self, max_children):
        self._children = []
        self.max_children = max_children
        self._aggregation = None

    @property
    def aggregation(self) -> Aggregation:
        if self._aggregation is None:
            return self.calculate_aggregation_from_children()
        else:
            assert self._aggregation
            return self._aggregation

    def calculate_aggregation_from_children(self) -> Aggregation:
        child_aggregations = [c.aggregation for c in self._children]
        return Aggregation.from_aggregations(child_aggregations)

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
            self._aggregation = self.calculate_aggregation_from_children()
            new_sibling = BtreeInternalNode(self.max_children)
            new_sibling.add_child(child_node)
            return new_sibling

    def __iter__(self):
        # TBD: At this moment, we iterate over all samples
        # recursing into child nodes. This might be counter-intuitive.
        for child in self._children:
            for sample in child:
                yield sample

    def select_range(self, selection_span: TimeSpan):
        """ Select a range of nodes falling between `begin` and `end` """
        assert self._children

        in_range_children = []
        full_span = self.aggregation.timespan
        if selection_span.overlaps(full_span):
            # In range, so:
            # Some overlap!
            # Find first node:
            for node in self._children:
                if selection_span.overlaps(node.aggregation.timespan):
                    in_range_children.append(node)

        return in_range_children

    def select_all(self):
        return self._children

    def last_value(self):
        return self._children[-1].last_value()


class BtreeLeaveNode(BtreeNode):
    """ A leave node in the B-tree.
    
    This node type actually contains raw observations.
    """

    def __init__(self, max_samples):
        self.samples = []
        self.max_samples = max_samples
        self._aggregation = None

    @property
    def aggregation(self) -> Aggregation:
        return self._aggregation

    @property
    def full(self):
        return len(self.samples) >= self.max_samples

    def _add_sample(self, sample):
        self.samples.append(sample)

        # Update metrics:
        aggregation = Aggregation.from_sample(sample)
        if self._aggregation:
            self._aggregation += aggregation
        else:
            self._aggregation = aggregation

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

    def select_range(self, selection_span: TimeSpan):
        """ Select a range of samples falling between `begin` and `end` """
        if not self.samples:
            return []

        full_span = self.aggregation.timespan
        in_range_samples = []
        if selection_span.overlaps(full_span):
            # In range, so:
            # Some overlap!
            # Find first node:
            for sample in self.samples:
                if selection_span.contains_timestamp(sample[0]):
                    in_range_samples.append(sample)
        else:
            # out of range
            pass

        return in_range_samples

    def select_all(self):
        """ Retrieve all samples in this node.
        """
        return self.samples

    def last_value(self):
        return self.samples[-1]
