with open("./puzzle_input.txt", "r") as file:
    puzzle_input = file.read().strip()

def input_to_matrix(str_in: str):
    matrix = [list(line) for line in str_in.splitlines()]
    rows = len(matrix)
    cols = len(matrix[0])
    
    return matrix, rows, cols

def test_str_for_result(string: str):
    if string == "XMAS" or string == "SAMX":
        return 1
    
    return 0

def count_horizontal_from_char(matrix: list[list[chr]], row: int, col: int):
    final_str = ""
    for i in range(0, 4):
        final_str += matrix[row][col + i]
    
    return test_str_for_result(final_str)

def count_vertical_from_char(matrix: list[list[chr]], row: int, col: int):
    final_str = ""
    for i in range(0,4):
        final_str += matrix[row + i][col]
        
    return test_str_for_result(final_str)

def count_diagonal_left_from_char(matrix: list[list[chr]], row: int, col: int):
    final_str = ""
    for i in range(0, 4):
        final_str = matrix[row + i][col + i]
    
    return test_str_for_result(final_str)

def count_diagonal_right_from_char(matrix: list[list[chr]], row: int, col: int):
    final_str = ""
    for i in range(0, 4):
        final_str = matrix[row + i][col - i]
    
    return test_str_for_result(final_str)

# works in rust
def count_xmas(wordsearch: str):
    matrix, rows, cols = input_to_matrix(wordsearch)
    count = 0
    
    for row in range(rows):
        for col in range(cols):
            # horizontal
            if col < cols - 3:
                count += count_horizontal_from_char(matrix, row, col)

            # vertical
            if row < rows - 3:
                count += count_vertical_from_char(matrix, row, col)
                
                # diagonal left
                if col < cols - 3:
                    count += count_diagonal_left_from_char(matrix, row, col)
                    
                # diagonal right
                if col >= 3:
                    count += count_diagonal_right_from_char(matrix, row, col)
    
    return count
    

total_occurrences = count_xmas(puzzle_input)
print(f"XMAS Count: {total_occurrences}")

def count_x_mas_patterns(wordsearch: str):
    matrix, rows, cols = input_to_matrix(wordsearch)
    
    count = 0
    
    r1 = range(1, rows - 1)
    r2 = range(1, cols - 1)

    for row in r1:
        for col in r2:
            top_left = matrix[row - 1][col - 1]
            top_right = matrix[row - 1][col + 1]
            center = matrix[row][col]
            bottom_left = matrix[row + 1][col - 1]
            bottom_right = matrix[row + 1][col + 1]

            mas_1 = [top_left, center, bottom_right]
            mas_2 = [bottom_left, center, top_right]
            
            mas1_match = mas_1 == list("MAS") or mas_1 == list("SAM")
            mas2_match = mas_2 == list("MAS") or mas_2 == list("SAM")
            
            if mas1_match and mas2_match:
                count += 1

    return count

xmas_total_occurrences = count_x_mas_patterns(puzzle_input)
print(f"X-MAS Count: {xmas_total_occurrences}")