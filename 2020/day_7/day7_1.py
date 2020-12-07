def get_formatted_data() -> []:
    with open('test.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]

    return lines


if __name__ == '__main__':
    pass
