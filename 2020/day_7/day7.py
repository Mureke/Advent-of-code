def get_formatted_data() -> {}:
    bags = {}
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]

        for line in lines:
            bag, contains = line.split('contain')
            contains_splitted = contains.split(',')
            bags[bag.split(' bag')[0]] = {}

            for inside_bag in contains_splitted:
                parts = inside_bag.lstrip().split(' ')
                if 'other' not in parts:
                    bags[bag.split(' bag')[0]][parts[1] + ' ' + parts[2]] = parts[0]

    return bags


def get_part_1_answer(bags):
    def has_gold_bag(bags, bag) -> bool:
        if type(bag) == dict:
            for key, bag in bag.items():
                if has_gold_bag(bags, bags[key]):
                    return True
                if key == 'shiny gold':
                    return True
        return False

    gold_bags = 0
    for key, value in bags.items():
        if has_gold_bag(bags, value):
            gold_bags += 1
    return gold_bags


def get_part_2_answer(bags):
    def get_bags_inside(bags, bag_name):
        bag_count = 0
        for bag_name2, count in bags[bag_name].items():
            bag_count += int(count)
            bag_count += get_bags_inside(bags, bag_name2) * int(count)
        return bag_count

    bag_amount = 0
    bag_amount += get_bags_inside(bags, 'shiny gold')
    return bag_amount


if __name__ == '__main__':
    data = get_formatted_data()
    print(get_part_1_answer(data))
    print(get_part_2_answer(data))
