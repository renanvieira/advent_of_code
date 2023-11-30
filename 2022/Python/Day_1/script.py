
def main(calories: list) -> None:
    calories_summed = []

    for elf_kcal in calories:
        calories_summed.append(sum(elf_kcal))

    calories_summed.sort(reverse=True)

    print(f"The elf carrying the most calories has: {calories_summed[0]}")
    print(f"The top 3 is {calories_summed[:3]}")
    top3 = sum(calories_summed[:3])
    print(f"The sum of the top 3 elfs is: {top3}")


if __name__ == "__main__":
    input_array = []
    local_arr = []
    last_line = None
    with open("input.txt", "rb") as f:
        for line in f:
            decoded_line = line.strip().decode("utf-8")
            if decoded_line:
                local_arr.append(int(decoded_line))
            else:
                input_array.append(local_arr)
                local_arr = []

    if local_arr:
        input_array.append(local_arr[:])
        local_arr.clear()

    main(input_array)



def test_main():
    input_array = []
    local_arr = []
    last_line = None
    with open("input.txt", "rb") as f:
        for line in f:
            decoded_line = line.strip().decode("utf-8")
            if decoded_line:
                local_arr.append(int(decoded_line))
            else:
                input_array.append(local_arr)
                local_arr = []

    if local_arr:
        input_array.append(local_arr[:])
        local_arr.clear()

    main(input_array)
