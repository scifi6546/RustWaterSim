import unittest
from py_watersim import Grid, GridStack,load_stack
import matplotlib.pyplot as plt

class Test(unittest.TestCase):
    def test_grid(self):
        g = Grid(10,10)
        s = g[0,0]
        self.assertTrue(abs(s) < 0.001)
        s1 = g[0,0] = 1.0
        self.assertTrue(abs(s1-1.0) < 0.001)

    def test_grid_stack_index(self):
        s = GridStack()
        s.push_grid(Grid(10,10))
        s[0,0,0] = 1.0
        self.assertEqual(s[0,0,0],1.0)
        self.assertEqual(s[0,0,1],0.0)
        s[0,0,0] = 1.0
        layer = s[0]
        row_a = layer[0]
        row_b = s[0,0]
        for i in range(0,10):
            diff = row_a[i]-row_b[i]
            self.assertTrue(abs(diff)<0.001)

    def test_plot(self):
        g = Grid(10,10)
        plt.imshow(g)
        plt.show()

    def test_grid_stack(self):
        s = GridStack()
        g = Grid(10,10)
        s.push_grid(g)
        s.save("stack")

        g_2 = load_stack("stack")




if __name__ == '__main__':
    unittest.main()
