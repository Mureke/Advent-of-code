

if __name__ == '__main__':
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        total_width = len(lines[0])
        total_height = len(lines)
        height = 0
        width = 0
        answer = 0
        while height < total_height:
            try:
                if lines[height][width] == '#':
                    answer += 1
            except IndexError as e:
                print('Index Error: ')
                print(width)
                print(height)
            width += 3
            height += 1

            if width >= total_width:
                width -= total_width

        print(answer)

