""" B-tree data structure.

Idea is to create summary levels on top of chunks of data.
"""

from .metrics import Metrics, sample_to_metric, merge_metrics


class Btree:
    def __init__(self):
        self.root_node = BtreeLeaveNode()

    def append(self, sample):
        """ Append a single sample. """
        new_root_sibling = self.root_node.append(sample)
        if new_root_sibling:
            print("new root!")
            old_root = self.root_node
            self.root_node = BtreeInternalNode()
            self.root_node.add_child(old_root)
            self.root_node.add_child(new_root_sibling)

    def extend(self, samples):
        """ Append a batch of samples. """
        # TODO: this can be improved for efficiency.
        for sample in samples:
            self.append(sample)

    def __len__(self):
        # Return total number of samples!
        return self.root_node.metrics.count

    def __iter__(self):
        for sample in self.root_node:
            yield sample

    def query(self, begin, end, min_count):
        """ Query this tree for some data between the given points.
        """
        assert begin < end
        selection_span = (begin, end)

        # Initial query result:
        nodes = self.root_node.select_range(selection_span)

        # Enhance resolution, while not enough samples.
        while nodes and len(nodes) < min_count and isinstance(nodes[0], BtreeNode):
            # Enhance resolution!
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
            nodes = new_nodes

        # Take metrics from internal nodes:
        if nodes and isinstance(nodes[0], BtreeNode):
            nodes = [n.metrics for n in nodes]

        return nodes


def overlap(span1, span2):
    """ Test if two spans overlap.

    Parameters are two spans, which are tuples of (begin, end).
    """
    assert span1[0] <= span1[1]
    assert span2[0] <= span2[1]

    return span1[0] <= span2[1] and span2[0] <= span1[1]


class BtreeNode:
    def __init__(self):
        pass

    def append(self, sample):
        raise NotImplementedError()

    def select_range(self, selection_span):
        raise NotImplementedError()

    def select_all(self):
        raise NotImplementedError()


class BtreeInternalNode(BtreeNode):
    MAX_CHILDREN = 5

    def __init__(self):
        self._children = []
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
        last_child = self._children[-1]
        new_child = last_child.append(sample)
        if new_child:
            if len(self._children) < self.MAX_CHILDREN:
                self.add_child(new_child)
            else:
                # We are full, calculate metrics!
                # print('new BtreeInternalNode sibling')
                self._metrics = self.calculate_metrics_from_children()
                new_sibling = BtreeInternalNode()
                new_sibling.add_child(new_child)
                return new_sibling
        else:
            pass  # We are Ok

    def __iter__(self):
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
    MAX_SAMPLES = 32

    def __init__(self):
        self.samples = []
        self._metrics = None

    @property
    def metrics(self):
        assert self._metrics is not None
        return self._metrics

    def _add_sample(self, sample):
        self.samples.append(sample)

        # Update metrics:
        metric = sample_to_metric(sample)
        if self._metrics:
            self._metrics = self._metrics + metric
        else:
            self._metrics = metric

    def append(self, sample):
        if len(self.samples) < self.MAX_SAMPLES:
            self._add_sample(sample)
        else:
            # print('new leave sibling!')
            new_sibling = BtreeLeaveNode()
            new_sibling._add_sample(sample)
            return new_sibling

    def __iter__(self):
        for sample in self.samples:
            yield sample

    def select_range(self, selection_span):
        """ Select a range of samples falling between `begin` and `end` """
        assert self.samples

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
        return self.samples
