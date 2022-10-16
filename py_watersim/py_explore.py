import matplotlib.pyplot as plt
import numpy as np
class Grid:
    def __init__(self):
        self.arr=[[0,1,2,3],[1,2,3,4]]
    def __iter__(self):
        return GridIter(self.arr,0)
    def get_arr(self):
        return np.array(self.arr)
class GridIter:
    def __init__(self,rows,idx):
        self.rows=rows
        self.idx=idx
    def __next__(self):
        if self.idx<len(self.rows):
            self.idx+=1
            return Row(self.rows[self.idx-1])
        else:
            raise StopIteration
    def __len__(self):
        return len(self.rows)
class Row:
    def __init__(self,row):
        self.row=row
    def __iter__(self):
        return RowIter(self.row)
class RowIter:
    def __init__(self,row):
        self.row=row
        self.idx=0
    def __next__(self):
        if self.idx<len(self.row):
            self.idx+=1
            return self.row[self.idx-1]
        else:
            raise StopIteration
    def __len__(self):
        return len(self.row)
t = Grid()
for r in t:
    print(r)
    for i in r:
        print(i)

print(dir([1]))
plt.imshow(t)