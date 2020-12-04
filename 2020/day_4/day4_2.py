import string

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
            one_cred += line + ' '
        credentials.append(one_cred)

        for cred in credentials:
            cred = list(filter(lambda x: True if x != '' else False, cred.split(' ')))
            cred_dict = {cred[i].split(':')[0]: cred[i].split(':')[1] for i in range(0, len(cred))}

            valid = True
            for req in required_fields:
                if req in cred_dict.keys():
                    if req == 'byr' and 2002 >= int(cred_dict[req]) >= 1920:
                        continue
                    elif req == 'iyr' and 2020 >= int(cred_dict[req]) >= 2010:
                        continue
                    elif req == 'eyr' and 2020 <= int(cred_dict[req]) <= 2030:
                        continue
                    elif req == 'hgt':
                        if 'cm' in cred_dict[req]:
                            height = int(cred_dict[req].replace('cm', ''))
                            if 150 <= height <= 193:
                                continue
                        if 'in' in cred_dict[req]:
                            height = int(cred_dict[req].replace('in', ''))
                            if 59 <= height <= 76:
                                continue
                    elif req == 'hcl' and cred_dict[req].startswith('#'):
                        color = cred_dict[req].replace('#', '')
                        if len(color) == 6 and all(x in string.hexdigits for x in color):
                            continue
                    elif req == 'ecl' and cred_dict[req] in ['amb', 'blu', 'brn', 'gry', 'grn','hzl', 'oth']:
                        continue
                    elif req == 'pid' and len(cred_dict['pid']) == 9 and cred_dict[req].isdigit():
                        continue
                    elif req == 'cid':
                        continue
                    valid = False
                else:
                    valid = False
            if valid:
                answer += 1

        print(answer)
