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
    closing_points_d2 = {")": 1, "]": 2, "}": 3, ">": 4}
    pairs = {"(": ")", "[": "]", "{": "}", "<": ">"}

    result = 0
    result2 = []
    for item in data:
        temp_res2 = 0
        is_valid, left_overs = check(item)
        if not is_valid and any(x in left_overs for x in closing.keys()):
            for i in left_overs:
                if i in closing.keys():
                    result += closing[i]
                    break

        if not is_valid and all(x in pairs.keys() for x in left_overs):
            for i in left_overs[::-1]:
                temp_res2 = temp_res2 * 5 + closing_points_d2[pairs[i]]
        if temp_res2 != 0:
            result2.append(temp_res2)
    result2.sort()
    print(f"Day10: 1: {result} 2. {result2[int(len(result2) / 2)]}")
