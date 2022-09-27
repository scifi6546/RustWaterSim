import matplotlib.pyplot as plt
import numpy as np

vel = np.load("velocity.np")
x = []
for i in range(0,vel.shape[0]):
    x.append(float(i))
x = np.array(x)
y = []
for i in range(0,vel.shape[0]):
    y.append(float(i))
y = np.array(y)
print(x.shape)
print(vel.shape)
u = []
v = []
for x in range(0,vel.shape[0]):
    for y in range(0,vel.shape[1]):
        vector = vel[x,y]
        u.append(vector[0])
        v.append(vector[1])
plt.quiver(x,y,u,v)
plt.show()
print("hello")
