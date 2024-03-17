def count_numbers_adjacent_to_symbols_simple(input_data):
    # Converting the input data to a matrix of characters
    matrix = [list(line.strip()) for line in input_data]
    rows = len(matrix)
    cols = len(matrix[0])

    # Defining adjacent neighbors
    adjacent_neighbors = [
        (-1, 0),  # Up
        (1, 0),   # Down
        (0, -1),  # Left
        (0, 1),   # Right
        (-1, -1), # Upper Left Diagonal
        (-1, 1),  # Upper Right Diagonal
        (1, -1),  # Lower Left Diagonal
        (1, 1),   # Lower Right Diagonal
    ]

    # Set to store unique numbers adjacent to symbols
    unique_numbers = set()

    # Iterate over each cell in the matrix
    for row in range(rows):
        for col in range(cols):
            if matrix[row][col].isdigit():
                # Check all neighbors of a digit to see if any is a symbol
                for neighbor in adjacent_neighbors:
                    nr, nc = row + neighbor[0], col + neighbor[1]
                    if 0 <= nr < rows and 0 <= nc < cols:
                        if matrix[nr][nc] not in '0123456789.':
                            # Extract the complete number
                            number_str = ''
                            while col < cols and matrix[row][col].isdigit():
                                number_str += matrix[row][col]
                                col += 1
                            unique_numbers.add(number_str)
                            break

    return len(unique_numbers)

file = open("Rust/src/data/day_3.txt", "r")
input_data = file.readlines()

# Counting the numbers adjacent to symbols using a simple iteration method
count_adjacent_numbers_simple = count_numbers_adjacent_to_symbols_simple(input_data)
print(count_adjacent_numbers_simple)

