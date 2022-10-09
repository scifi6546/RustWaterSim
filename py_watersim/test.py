import unittest
from py_watersim import Grid


class Test(unittest.TestCase):
    def test_grid(self):
        g = Grid(10,10)
        s = g[0,0]
        self.assertTrue(abs(s) < 0.001)
        s1 = g[0,0] = 1.0
        self.assertTrue(abs(s1-1.0) < 0.001)


if __name__ == '__main__':
    unittest.main()
