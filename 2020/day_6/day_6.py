def get_formatted_data() -> []:
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        formatted_data = []
        one_group_answer = []
        for line in lines:
            if line == '':
                formatted_data.append(one_group_answer)
                one_group_answer = []
            else:
                one_group_answer.append(line)
        formatted_data.append(one_group_answer)
    return formatted_data


if __name__ == '__main__':
    print('Part 1: ', sum(map(lambda x: len(set(''.join(x))), get_formatted_data())))
    print('Part 2: ', sum(map(lambda x: len(set.intersection(*[set(y) for y in x])), get_formatted_data())))

