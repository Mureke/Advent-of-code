import copy


def get_formatted_data() -> []:
    instructions = []
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        for line in lines:
            instruction, val = line.split(' ')
            instructions.append({'instruction': instruction, 'value': int(val)})

    return instructions


class Machine:
    pointer = 0
    acc = 0
    instructions = []
    instruction_history = []

    def __init__(self, instructions):
        self.instructions = instructions
        self.instruction_history = []
        self.pointer = 0
        self.acc = 0

    def next(self):
        self.instruction_history.append(self.pointer)

        inst = self.instructions[self.pointer]
        instruction = inst['instruction']
        val = inst['value']

        if instruction == 'acc':
            self.acc += val
            self.pointer += 1
        elif instruction == 'jmp':
            self.pointer += val
        else:
            self.pointer += 1


def change_instruction_value(instructions, last_tested):
    for index, val in enumerate(instructions[last_tested:]):
        index = last_tested + index
        if val['instruction'] == 'nop':
            instructions[index]['instruction'] = 'jmp'
            last_tested = index
            break
        if val['instruction'] == 'jmp':
            instructions[index]['instruction'] = 'nop'
            last_tested = index
            break
    last_tested += 1
    return instructions, last_tested


def get_answer(instructions):
    machine = Machine(instructions)
    last_tested = 0
    a1 = None
    while machine.pointer < (len(instructions)):
        machine.next()
        if machine.pointer in machine.instruction_history:
            if not a1:
                a1 = machine.acc
            new_instructions, last_tested = change_instruction_value(copy.deepcopy(instructions), last_tested)
            machine = Machine(new_instructions)
    return a1, machine.acc


if __name__ == '__main__':
    data = get_formatted_data()
    answer1, answer2 = get_answer(data)
    print(f'Part 1 answer: {answer1}')
    print(f'Part 2 answer: {answer2}')
