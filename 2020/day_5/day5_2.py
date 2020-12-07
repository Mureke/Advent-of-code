import numpy as np

ROWS_START = 0
ROWS_END = 127
COLS_START = 0
COLS_END = 7


def decode_line(line):
    row_start = ROWS_START
    row_end = ROWS_END
    cols_start = COLS_START
    cols_end = COLS_END
    for i in line:
        if i == 'F':
            row_end -= int((row_end - row_start) / 2) + 1
        elif i == 'B':
            row_start = int((row_end + row_start) / 2) + 1
        elif i == 'L':
            cols_end -= int((cols_end - cols_start) / 2) + 1
        elif i == 'R':
            cols_start = int((cols_end + cols_start) / 2) + 1

    if cols_start == cols_end and row_end == row_end:
        return row_start, cols_start
    raise Exception('Calc error')


if __name__ == '__main__':
    answer = 0
    seats = np.zeros((ROWS_END + 1, COLS_END + 1))
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        for line in lines:
            row, col = decode_line(line)
            seats[row][col] = 1

        indexes = np.where(seats == 0)

        for key, value in enumerate(indexes[0]):
            col = value
            row = indexes[1][key]
            seat_id = col * 8 + row

            try:
                if seats[col][row + 1] == 1 and seats[col][row + -1]:
                    answer = seat_id
            except IndexError:
                continue

        print(answer)
