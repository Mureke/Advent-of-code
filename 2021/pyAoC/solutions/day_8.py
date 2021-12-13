import os


def solve_day_1(data):
    occurrences = 0
    for item in data:
        occurrences += len(list(filter(lambda x: len(x) in (2, 4, 3, 7), item.split(" "))))

    return occurrences

def solve_day_2(data):
    numbers = {
        0: [],
        1: [],
        2: [],
        3: [],
        4: [],
        5: [],
        6: [],
        7: [],
        8: [],
        9: [],
    }
    for row in data.split("\n"):
        input, output = row.split("|")
        full = input + output
        temp2 = list(filter(lambda x: len(x) in (2, 4, 3, 7), full.split(" ")))
        for item in temp2:
            if len(item) == 2:
                numbers[1] = list(item)
            elif len(item) == 3:
                numbers[7] = list(item)
            elif len(item) == 4:
                numbers[4] = list(item)
            elif len(item) == 7:
                numbers[8] = list(item)

        for item in full.split(" "):
            if len(item) == 5: # 3 or 5 or 2
                if numbers[1] in list(item):
                    numbers[3] = list(item)
                else:
                    numbers[5] = list(item)
    return "Not implemented"


def solve():
    with open(os.path.abspath('data/day_8.txt')) as file:
        data = file.read()

    day_1_data = []
    for i in data.replace('|', '\n').split('\n'):
        if len(i.split(' ')) == 5:
            day_1_data.append(i)
    print(f"Day 8: 1: {solve_day_1(day_1_data)} 2: {solve_day_2(data)}")
