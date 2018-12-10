import numpy as np
from re import findall
import itertools
import matplotlib.pyplot as plt


def refactor_data(arr=[]):
    refactored = []
    for x, y, vx, vy in arr:
        area_object = {
                'x': int(x),
                'y': int(y),
                'vx': int(vx),
                'vy': int(vy)
            }
        refactored.append(area_object)

    return refactored


def create_matrix(obj):
    counter = 0
    while True:
        ready = True
        for o in obj:
            o['y'] += o['vy']
            o['x'] += o['vx']

            if o['y'] < 0 or o['y'] > 170 or o['x'] < 0 or o['x'] > 280:
                ready = False

        counter += 1
        if ready is True:
            break

    for _ in itertools.repeat(None, 10000):
        draw_image = True
        area = np.full((170, 280), 0)
        for o in obj:
            try:
                area[int(o['y'])][int(o['x'])] = 1
                o['y'] += o['vy']
                o['x'] += o['vx']

            except Exception:
                draw_image = False
                continue

        if draw_image is True:
            plt.imshow(area)
            plt.show()
            print(counter)
        counter += 1


if __name__ == '__main__':
    np.set_printoptions(threshold=np.nan)
    plt.style.use('classic')

    initial_data = [map(int, findall(r'-?\d+', i)) for i in open('inputs.txt').readlines()]
    data = refactor_data(initial_data)
    create_matrix(data)
