

if __name__ == '__main__':
    with open('data.txt', 'r') as f:
        lines = list(map(int, f.readlines()))

        for i in lines:
            if (2020 - i) in lines:
                print('Answer to 1: ', i * (2020 - i))
            for j in lines:
                if (2020 - i - j) in lines:
                    print('Answer to 2: ', i * j * (2020-i-j))
