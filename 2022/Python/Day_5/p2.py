
from collections import deque
import re


def main(raw_crate_drawing: list[str], operations: list[str]) -> None:
    crate_schema: list[deque] = []

    # Preparing the stacks and initializing queues
    number_of_crates = len(raw_crate_drawing[0]) // 4

    crate_schema = [deque() for _ in range(number_of_crates+1)]

    # Iterating over the crate drawings and adding to the queue
    print(raw_crate_drawing[-2])
    for idx, crate_row in enumerate(raw_crate_drawing[:-1]):
        crates = crate_row[1:-1:4]
        for crate, item in zip(crate_schema, crates):
            if item != '' and item != ' ':
                crate.appendleft(item)

    # Performing Operations
    OP_REGEX = r'\d+'

    for op in operations[:-1]:
        num_containers, stack_from, stack_to = re.findall(OP_REGEX, op)
        num_containers = int(num_containers)
        stack_from = int(stack_from) - 1
        stack_to = int(stack_to) - 1

        print(f"Moving {num_containers} from {stack_from} to {stack_to}")

        # Changes for Part 2
        containers_moving = reversed([crate_schema[stack_from].pop()
                                      for _ in range(num_containers)])

        crate_schema[stack_to].extend(containers_moving)

    top_crates = [c[-1] for c in crate_schema]
    top_crates_str = "".join(top_crates)

    print(f"Top Crates: {top_crates} | {top_crates_str}")


if __name__ == "__main__":
    with open("input", "rb") as f:
        lines = f.read()
        lines = lines.decode("utf8")
        lines = lines.split("\n")

        empty_line_index = lines.index("")

        raw_crates = lines[0: empty_line_index]
        operations = lines[empty_line_index+1: len(lines)]

        main(raw_crates, operations)
