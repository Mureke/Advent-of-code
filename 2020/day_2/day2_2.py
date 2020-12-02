

if __name__ == '__main__':
    with open('data.txt', 'r') as f:
        lines = f.readlines()
        a1 = 0
        for i in lines:
            splitted = i.split(':')
            num_range = splitted[0]
            password = splitted[1].strip(' ')
            val1 = int(num_range.split('-')[0]) - 1
            max_val_and_char = num_range.split('-')[1].split(' ')
            val2 = int(max_val_and_char[0]) - 1
            char = max_val_and_char[1]

            first_char = password[val1]
            second_char = password[val2]
            if (first_char == char and second_char != char) or (second_char == char and first_char != char):
                a1 += 1

        print('Answer to 2: ', a1)


