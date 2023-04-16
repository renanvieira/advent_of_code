from collections import Counter, OrderedDict
import string





def main(rucksacks: list[list[str]], priority) -> None:
    prioritized_items = []

    priority_sum = 0
    for sack in rucksacks:
        first_compartiment = set(sack[0])
        second_compartiment = set(sack[1])

        common_items = first_compartiment.intersection(second_compartiment)

        for item in common_items:
            priority_sum += int(priority[item])
            prioritized_items.append((item, priority[item]))
        
    print(f"Prioritized Items: {prioritized_items}")
    print(f"Sum of Priorities: {priority_sum}")
    print(prioritized_items)


if __name__ == "__main__":      
    priority = {letter: index for index, letter in enumerate(string.ascii_letters, start=1)}

    with open("/Users/renanvieira/Projects/advent_of_code/Python/Day_3/input", "rb") as f:
        data = []
        for line in f:
            line_str = line.strip().decode("utf-8")
            line_length = len(line_str)
            half_line = line_length // 2
            first_compartiment = line_str[:half_line]
            second_compartiment = line_str[half_line:]

            data.append([first_compartiment, second_compartiment])
          
        main(data, priority)
