import unittest
from py_watersim import Grid, GridStack,load_stack


class Test(unittest.TestCase):
    def test_grid(self):
        g = Grid(10,10)
        s = g[0,0]
        self.assertTrue(abs(s) < 0.001)
        s1 = g[0,0] = 1.0
        self.assertTrue(abs(s1-1.0) < 0.001)

    def test_grid_stack(self):
        s = GridStack()
        g = Grid(10,10)
        s.push_grid(g)
        s.save("stack")

        g_2 = load_stack("stack")



if __name__ == '__main__':
    unittest.main()
