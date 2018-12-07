from fuzzywuzzy import fuzz


def get_data():
    text_file = open("inputs.txt", "r")
    lines = text_file.readlines()
    text_file.close()

    return lines


if __name__ == '__main__':
    data = get_data()
    data2 = data
    highest_score = 0
    index = 0
    for value in data:
        for value2 in data2:
            score = fuzz.token_sort_ratio(value, value2)
            if score > highest_score and value is not value2:
                correct_boxes = []
                highest_score = score
                correct_boxes.append(value2)
                correct_boxes.append(value)
        index += 1

    print(correct_boxes)
