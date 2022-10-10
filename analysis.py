import math

import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import numpy as np
from pathlib import Path

SKIP_INTERVAL = 5
SCALE = 0.1


fig = plt.figure(figsize=(14,18))
ax = plt.axes()

DATA_LEN = 0

def make_quiver():
    x_arr = []
    y_arr = []
    u = []
    v = []
    slope_path = Path("debug_data/velocity_10.npz")
    data = np.load(slope_path)["0"][0]
    for x in range(0,data.shape[0]):
        for y in range(0,data.shape[1]):
            if x % SKIP_INTERVAL == 0 and y % SKIP_INTERVAL == 0:
                x_arr.append(float(x))
                y_arr.append(float(y))
                u.append(0.01)
                v.append(0.01)
    DATA_LEN = len(x_arr)
    return ax.quiver(y_arr,x_arr,u,v,scale=1.0, width=0.001)


def make_water_img():
    water_path = Path("debug_data/water_10.npz")
    data = np.load(str(water_path))["0"][0]
    return ax.imshow(data)


quiver = make_quiver()
img = make_water_img()

def init():
    return quiver


def print_velocity(file_path,offset):
    data = np.load(file_path)
    data = data["0"][offset]
    print(data.shape)
    u = []
    v = []

    for x in range(0, data.shape[0]):
        for y in range(0, data.shape[1]):
            if x % SKIP_INTERVAL == 0 and y % SKIP_INTERVAL == 0:
                vector = data[x, y]

                u.append(vector[0])
                v.append(vector[1])

    u = np.array(u)
    v = np.array(v)
    mags = np.sqrt(np.power(u, 2.0)+np.power(v, 2.0))
    print("max velocity: {}".format(np.max(mags)))

    quiver.set_UVC(v , u)
    return quiver


def animate(i):
    rounded_i = int(math.ceil(i/10.0)) * 10
    ten_offset = i%10
    print("i: {} rounded i: {} offset: {}".format(i, rounded_i, ten_offset))
    slope_path = Path("debug_data/velocity_{}.npz".format(rounded_i))
    if slope_path.exists():
        water = np.load(str(Path("debug_data/water_{}.npz".format(rounded_i))))
        water = water["0"][ten_offset]
        print(water.shape)
        img.set_data(water)

        print(i)
        q = print_velocity(slope_path,ten_offset)
        return q
    else:
        print("path: {} not found".format(slope_path))
        return quiver



print("fig made")
anim = FuncAnimation(fig, animate, init_func=init, frames=340, interval=20)
anim.save("test.mp4")