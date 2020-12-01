

if __name__ == '__main__':
    with open('data.txt', 'r') as f:
        lines = list(map(int, f.readlines()))
        a1 = None
        a2 = None
        for i in lines:
            if not a1 and (2020 - i) in lines:
                a1 = i * (2020 - i)
            for j in lines:
                if not a2 and (2020 - i - j) in lines:
                    a2 = i * j * (2020-i-j)
            if a1 and a2:
                break

        print('Answer to 1: ', a1)
        print('Answer to 2: ', a2)


