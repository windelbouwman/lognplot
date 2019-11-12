import unittest
from chart1.btree import overlap, Btree


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
        self.assertEqual([(1, 9), (5, 11), (9, 9)], tree.query(0, 30, 1))
        self.assertEqual([(5, 11), (9, 9)], tree.query(5, 30, 1))
        self.assertEqual([], tree.query(20, 30, 1))


class OverlapTestCase(unittest.TestCase):
    def test_no_overlap1(self):
        self.assertFalse(overlap((1, 3), (6, 7)))

    def test_no_overlap2(self):
        self.assertFalse(overlap((10, 13), (6, 7)))

    def test_overlap1(self):
        self.assertTrue(overlap((1, 8), (6, 17)))

    def test_overlap2(self):
        self.assertTrue(overlap((1, 19), (6, 17)))

    def test_overlap3(self):
        self.assertTrue(overlap((8, 19), (6, 17)))

    def test_overlap1_reversed(self):
        self.assertTrue(overlap((6, 17), (1, 8)))

    def test_overlap2_reversed(self):
        self.assertTrue(overlap((6, 17), (1, 19)))

    def test_overlap3_reversed(self):
        self.assertTrue(overlap((6, 17), (8, 19)))
