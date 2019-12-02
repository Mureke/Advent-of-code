from data_reader import get_data
import numpy as np


def refactor_data(arr=[]):
    refactored = []
    for row in arr:
        elf_id, useless, position, size = row.split()
        top = position.split(',')[0]
        left = position.split(',')[1]
        width = size.split('x')[0]
        height = size.split('x')[1]
        area_object = {
                'id': int(elf_id[1:]),
                'x': int(top),
                'y': int(left[:-1]),
                'width': int(width),
                'height': int(height)
            }
        refactored.append(area_object)

    return refactored


def create_matrix(obj):
    fab = np.zeros((1000, 1000), dtype=np.int)
    for o in obj:
        area = fab[o['y']: o['y'] + o['height'], o['x']: o['x'] + o['width']]
        area[:] = area + 1

    return fab


if __name__ == '__main__':
    initial_data = get_data(False)
    data = refactor_data(initial_data)
    fabric = create_matrix(data)
    print(np.sum(np.where(fabric > 1, 1, 0)))
