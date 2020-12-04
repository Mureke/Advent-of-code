

if __name__ == '__main__':
    answer = 0
    required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
    with open('data.txt', 'r') as f:
        lines = [line.strip() for line in f.readlines()]
        credentials = []
        one_cred = ''
        for line in lines:
            if line == '':
                credentials.append(one_cred)
                one_cred = ''
            one_cred += line
        credentials.append(one_cred)

        for cred in credentials:
            valid = True
            for req in required_fields:
                if req not in cred:
                    valid = False
            if valid:
                answer += 1

        print(answer)
