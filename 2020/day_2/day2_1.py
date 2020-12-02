

if __name__ == '__main__':
    with open('data.txt', 'r') as f:
        lines = f.readlines()
        a1 = 0
        for i in lines:
            splitted = i.split(':')
            num_range = splitted[0]
            password = splitted[1]
            min_val = int(num_range.split('-')[0])
            max_val_and_char = num_range.split('-')[1].split(' ')
            max_val = int(max_val_and_char[0])
            char = max_val_and_char[1]

            count = sum(part == char for part in password)

            if min_val <= count <= max_val:
                a1 += 1
        print('Answer to 1: ', a1)


