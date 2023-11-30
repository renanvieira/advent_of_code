from collections import Counter, OrderedDict
import string





def main(rucksacks: list[list[str]], priority) -> None:


    group_priority_sum = 0
    for group in rucksacks:
        unique_items_in_group = [set(l) for l in group]
        intersection = set.intersection(*unique_items_in_group)

        for item in intersection:
            group_priority_sum += priority[item]

    print(f"Priority sum: {group_priority_sum}")

if __name__ == "__main__":      
    priority = {letter: index for index, letter in enumerate(string.ascii_letters, start=1)}

    data = []
    group = [] 
    with open("/Users/renanvieira/Projects/advent_of_code/Python/Day_3/input", "rb") as f:
        for line in f:
            line_str = line.strip().decode("utf-8")
            group.append(line_str)

            if len(group) == 3:
                data.append(group)
                group = []
        
          
        main(data, priority)
