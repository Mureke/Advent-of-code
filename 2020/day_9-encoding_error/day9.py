PREAMBLE = 25


def get_formatted_data() -> []:
    with open('data.txt', 'r') as f:
        return [int(line.strip()) for line in f.readlines()]


def get_answer1(numbers):
    current = PREAMBLE
    valid = True
    current_number = numbers[current]
    while valid:
        current_number = numbers[current]
        current_num_range = numbers[current - PREAMBLE:current]
        valid = False
        for number in numbers[current - PREAMBLE:current]:
            if current_number - number in current_num_range:
                valid = True
        current += 1

    return current_number


def get_answer2(numbers, invalid_number):
    start = 0
    i = 0
    count = 0
    while True:
        for number in numbers[start:]:
            count += number
            i += 1

            if count == invalid_number:
                num_range = numbers[start:start + i]
                return max(num_range) + min(num_range)
            elif count > invalid_number:
                start += 1
                i = 0
                count = 0
                break


if __name__ == '__main__':
    data = get_formatted_data()
    answer = get_answer1(data)
    print(answer)
    print(get_answer2(data, answer))

# > 2893044