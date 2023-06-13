
def main(input: list[str]) -> None:
    overlappingTasks = 0

    for section in input:
        pairs = section.split(',')

        firstPair = pairs[0]
        secondPair = pairs[1]

        firstAssignmentArrayRange = firstPair.split('-')

        firstAssignmentArrayRange = [int(s) for s in firstAssignmentArrayRange]
        firstAssignmentRange = set(range(
            firstAssignmentArrayRange[0], firstAssignmentArrayRange[1]+1))

        secondAssingmentArrayRange = secondPair.split('-')
        secondAssingmentArrayRange = [int(s)
                                      for s in secondAssingmentArrayRange]
        secondAssingmentRange = set(range(
            secondAssingmentArrayRange[0], secondAssingmentArrayRange[1]+1))

        smaller = firstAssignmentRange if len(firstAssignmentRange) < len(
            secondAssingmentRange) else secondAssingmentRange

        bigger = secondAssingmentRange if len(secondAssingmentRange) > len(
            firstAssignmentRange) else firstAssignmentRange

        if smaller.intersection(bigger):
            overlappingTasks += 1
            # print("+ Overlapped")

        # print(f"Pair: {pairs}")
        # print(f"First Assignment Range: {firstAssignmentRange}")
        # print(f"Second Assingment Range: {secondAssingmentRange}")
        # print("-"*25)

    print(f"Number of overlapping tasks: {overlappingTasks}")


if __name__ == "__main__":
    with open("input", "rb") as f:
        lines = f.readlines()
        lines = [b.decode('utf8').strip() for b in lines]
        main(lines)
