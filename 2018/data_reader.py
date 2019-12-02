def get_data(test=False):
    file = 'inputs.txt'
    if test:
        file = 'sample.txt'
    text_file = open(file, "r")
    lines = text_file.read().splitlines()
    text_file.close()

    return lines
