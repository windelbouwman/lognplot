import unittest
from lognplot.tsdb import Btree
from lognplot.time import TimeSpan


class BtreeTestCase(unittest.TestCase):
    def test_fill_and_iterate(self):
        samples = list((t, t) for t in range(10000))
        tree = Btree()
        tree.extend(samples)
        self.assertEqual(10000, len(tree))
        self.assertEqual(list(tree), samples)

    def test_query(self):
        """ Test the behavior of b-tree-querying. """
        tree = Btree()
        tree.append((1, 9))
        tree.append((5, 11))
        tree.append((9, 9))
        self.assertEqual([(1, 9), (5, 11), (9, 9)], tree.query(TimeSpan(0, 30), 1))
        self.assertEqual([(5, 11), (9, 9)], tree.query(TimeSpan(5, 30), 1))
        self.assertEqual([], tree.query(TimeSpan(20, 30), 1))


def test_fill_rate(benchmark):
    benchmark(fill_run)


def fill_run():
    samples = list((t, t) for t in range(10000))
    tree = Btree()
    tree.extend(samples)
