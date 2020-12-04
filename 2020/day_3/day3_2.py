

def ride_slope(right, down):
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        total_width = len(lines[0])
        total_height = len(lines)
        height = 0
        width = 0
        trees = 0
        while height < total_height:
            try:
                if lines[height][width] == '#':
                    trees += 1
            except IndexError as e:
                print('Index Error: ')
                print(width)
                print(height)
            width += right
            height += down

            if width >= total_width:
                width -= total_width

        return trees


if __name__ == '__main__':
    answer = []
    slopes = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2]
    ]
    for slope in slopes:
        total = ride_slope(slope[0], slope[1])
        print(f'Encountered {total} trees when riding {slope[0]} right and {slope[1]} down.')
        answer.append(total)

    a = answer[0]
    for item in answer[1:]:
        a *= item

    print(f'Answer: {a}')

# > 23028

