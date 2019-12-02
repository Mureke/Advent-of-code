from collections import Counter


def get_data():
    text_file = open("inputs.txt", "r")
    lines = text_file.readlines()
    text_file.close()

    return lines


if __name__ == '__main__':
    data = get_data()
    two = 0
    three = 0
    for row in data:
        values = Counter(row)
        if list(values.values()).count(2) is not 0:
            two += 1

        if list(values.values()).count(3) is not 0:
            three += 1


    checksum = two * three
    print(str(two) + ' * ' + str(three) + ' = ' + str(checksum))
    print(checksum)
