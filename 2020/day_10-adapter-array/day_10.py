def get_formatted_data() -> []:
    with open('data.txt', 'r') as f:
        return [int(line.strip()) for line in f.readlines()]


def get_answer(numbers):
    numbers.append(max(numbers) + 3)
    one_diff = 0
    three_diff = 0
    for i, num in enumerate(numbers.sort()):
        print(numbers)
    return 0


if __name__ == '__main__':
    data = get_formatted_data()
    answer = get_answer(data)
