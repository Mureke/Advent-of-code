from data_reader import get_data
import numpy as nu


def refactor_data(arr=[]):
    refactored = []
    for row in arr:
        elf_id, useless, position, size = row.split()
        top = position.split(',')[0]
        left = position.split(',')[1]
        width = size.split('x')[0]
        height = size.split('x')[1]
        area_object = {
            elf_id[1:]: {
                "top": top,
                "left": left,
                "width": width,
                "height": height,
            }
        }
        refactored.append(area_object)

    return refactored


if __name__ == '__main__':
    intial_data = get_data(False)
    data = refactor_data(intial_data)

