


from math import *
import os
import sys

def manit(lower = 0, upper = 1, maxsplit = 8):
    seen: set[tuple[int, int, int]] = set()
    v = []
    space = np.linspace(lower, upper, maxsplit)
    for acc in space:
        x = sin(acc)
        for acc_y in range(cnt):
            for acc_z in range(cnt):
                x, y, z = 0, 0, 0
                if (floor(x), floor(y), floor(z)) not in seen()
    return np.asfarray(v)

from pprint import pprint
import matplotlib.pyplot as plt
import numpy as np

## 100 linearly spaced numbers
##acc = np.linspace(0,5,8192)

# acc = np.linspace(0,5,8)
#
# print(f'[{type(acc)}]')
# pprint(acc)

ff = manit()
print(ff)
sys.exit(os.EX_OK)

# the function, which is y = x^2 here
x = acc
y = acc * 2

# setting the axes at the centre
fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
ax.spines['left'].set_position('center')
ax.spines['bottom'].set_position('zero')
ax.spines['right'].set_color('none')
ax.spines['top'].set_color('none')
ax.xaxis.set_ticks_position('bottom')
ax.yaxis.set_ticks_position('left')

# plot the function
plt.plot(x,y, 'r')

# show the plot
plt.show()
