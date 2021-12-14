import os


def check(data: str):
    brackets = ["()", "[]", "{}", "<>"]

    while any(x in data for x in brackets):
        for br in brackets:
            data = data.replace(br, '')

    return (not data, data)


def solve():
    with open(os.path.abspath('data/day_10.txt')) as file:
        data = file.read()

    data = data.split("\n")

    closing = {")": 3, "]": 57, "}": 1197, ">": 25137}
    result = 0
    for item in data:
        is_valid, left_overs = check(item)
        if not is_valid and any(x in left_overs for x in closing.keys()):
            for i in left_overs:
                if i in closing.keys():
                    print(closing[i])
                    result += closing[i]
                    break

    print(f"Day10: 1: {result}")
