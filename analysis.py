import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import numpy as np
from pathlib import Path
vel = np.load("velocity.np")

slope_path = Path("debug_data/velocity_10.np")
data = np.load(slope_path)
x_arr = []
y_arr = []
u = []
v = []
for x in range(0,data.shape[0]):
    for y in range(0,data.shape[1]):
        if x%10==0 and y%10==0:
            x_arr.append(float(x))
            y_arr.append(float(y))
            u.append(0.0)
            v.append(0.0)
quiver = plt.quiver(x_arr,y_arr,u,v)
print("made quiver")
def init():
    return quiver
def print_velocity(file_path):
    SCALE=1.0
    data = np.load(file_path)
    x_arr = []
    y_arr = []
    u = []
    v = []
    for x in range(0,data.shape[0]):
        for y in range(0,data.shape[1]):
            if x % 10 == 0 and y % 10 == 0:
                x_arr.append(x)
                y_arr.append(y)
                vector = SCALE*vel[x,y]

                u.append(vector[0])
                v.append(vector[1])
    x_arr = np.array(x_arr)
    y_arr = np.array(y_arr)
    u = np.array(u)
    v = np.array(v)

    quiver.set_UVC(u,v)
    return quiver


def animate(i):
    slope_path = Path("debug_data/velocity_{}.np".format(i*10))
    print(i)
    if slope_path.exists():
        q = print_velocity(slope_path)
        return q
    else:
        return quiver


fig = plt.figure()
print("fig made")
anim = FuncAnimation(fig, animate, init_func=init, frames=13,interval=1)
anim.save("test.mp4")