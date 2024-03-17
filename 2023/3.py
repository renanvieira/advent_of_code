import re

with open("Rust/src/data/day_3.txt") as f:
    content = [line.rsplit()[0] for line in f]

def verify_horizontal(string, start, end):
    if start == 0:
        string = string[start : end + 1]
    elif len(string) < end + 1:
        string = string[start - 1 : end]
    else:
        string = string[start - 1 : end + 1]

    pattern = re.compile("[^\w\.]")
    symbol = re.findall(pattern, string)
    if len(symbol) > 0:
        return True
    else:
        return False


def verify_diagonals(string, start, end):
    if start == 0:
        string = string[start : end + 1]
    elif len(string) < end + 1:
        string = string[start - 1 : end]
    else:
        string = string[start - 1 : end + 1]
    pattern = re.compile("[^\w\.]")
    symbols = re.findall(pattern, string)
    if len(symbols) > 0:
        return True
    else:
        return False


schematic_engine = []
example = content
for i, line in enumerate(example):
    pattern = re.compile("\d+")
    check = re.finditer(pattern, line)
    for number in check:
        start = number.start()
        end = number.end()

        if i != 0:
            up = example[i - 1]
        else:
            up = "..." * 50
        if i != (len(example) - 1):
            down = example[i + 1]
        else:
            down = "..." * 50

        if (
            verify_diagonals(up, start, end)
            or verify_diagonals(down, start, end)
            or verify_horizontal(line, start, end)
        ):
            schematic_engine.append(number.group())

schematic_engine = [int(number) for number in schematic_engine]
print(sorted(schematic_engine))
print(sum(schematic_engine))


# Part 2
def verfiy_horizontal(string, start):
    left = string[:start]
    right = string[start + 1 :]

    pattern_left = re.compile("(\d+)$")
    numbers_left = re.findall(pattern_left, left)

    pattern_right = re.compile("^(\d+)")
    numbers_right = re.findall(pattern_right, right)

    return numbers_left + numbers_right


def verify_diagonal(string, start):
    pattern = re.compile("\d+")
    numbers = re.finditer(pattern, string)

    adjacent = []
    for number in numbers:
        number_range = range(number.start(), number.end())
        if (
            (start in number_range)
            or (start - 1 in number_range)
            or (start + 1 in number_range)
        ):
            adjacent.append(number.group())

    return adjacent


gear_ratios = []
example = content
for i, line in enumerate(example):
    pattern = re.compile("\*")
    check = re.finditer(pattern, line)
    for asteriks in check:
        start = asteriks.start()
        end = asteriks.end()

        if i != 0:
            up = example[i - 1]
        else:
            up = "..." * 50
        if i != (len(example) - 1):
            down = example[i + 1]
        else:
            down = "..." * 50

        numbers_h = verfiy_horizontal(line, start)
        numbers_u = verify_diagonal(up, start)
        numbers_d = verify_diagonal(down, start)

        numbers = numbers_h + numbers_u + numbers_d
        if len(numbers) == 2:
            gear_ratios.append(int(numbers[0]) * int(numbers[1]))


print(sum(gear_ratios))
