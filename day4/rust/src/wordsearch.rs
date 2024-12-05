fn wordsearch_to_matrix(wordsearch: &String) -> Vec<Vec<char>> {
    let raw_vec: Vec<&str> = wordsearch.lines().collect();

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for row in raw_vec {
        matrix.push(row.chars().collect());
    }

    return matrix;
}

fn count_horizontal_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut chars: Vec<char> = Vec::new();
    for i in 0..4 {
        chars.push(matrix[row][col + i]);
    }

    let joined_str: String = chars.iter().collect();
    if joined_str == "XMAS" || joined_str == "SAMX" {
        return 1;
    }

    return 0;
}

fn count_vertical_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut chars: Vec<char> = Vec::new();
    for i in 0..4 {
        chars.push(matrix[row + i][col]);
    }

    let joined_str: String = chars.iter().collect();
    if joined_str == "XMAS" || joined_str == "SAMX" {
        return 1;
    }

    return 0;
}

fn count_diagonal_left_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut chars: Vec<char> = Vec::new();
    for i in 0..4 {
        chars.push(matrix[row + i][col + i]);
    }

    let joined_str: String = chars.iter().collect();
    if joined_str == "XMAS" || joined_str == "SAMX" {
        return 1;
    }

    return 0;
}

fn count_diagonal_right_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut chars: Vec<char> = Vec::new();
    for i in 0..4 {
        chars.push(matrix[row + i][col - i]);
    }

    let joined_str: String = chars.iter().collect();
    if joined_str == "XMAS" || joined_str == "SAMX" {
        return 1;
    }

    return 0;
}

fn count_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut count = 0;

    // horizontal
    if col < matrix[row].len() - 3 {
        count += count_horizontal_from_char(matrix, row, col);
    }

    // vertical
    if row < matrix.len() - 3 {
        count += count_vertical_from_char(matrix, row, col);

        // diagonal left
        if col < matrix[row].len() - 3 {
            count += count_diagonal_left_from_char(matrix, row, col);
        }

        // diagonal right
        if col >= 3 {
            count += count_diagonal_right_from_char(matrix, row, col);
        }
    }

    return count;
}

pub fn count_xmas(wordsearch: &str) -> i32 {
    let matrix = wordsearch_to_matrix(&String::from(wordsearch));

    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            count += count_from_char(&matrix, i, j);
        }
    }

    return count;
}

// works in python
pub fn count_x_mas(wordsearch: &str) -> i32 {
    let mut count = 0;

    let matrix = wordsearch_to_matrix(&String::from(wordsearch));

    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let top_left = matrix[row - 1][col - 1];
            let top_right = matrix[row - 1][col + 1];
            let center = matrix[row][col];
            let bottom_left = matrix[row + 1][col - 1];
            let bottom_right = matrix[row + 1][col + 1];

            let mas1: String = vec![top_left, center, top_right].into_iter().collect();
            let mas2: String = vec![bottom_left, center, bottom_right]
                .into_iter()
                .collect();

            let mas1_match = mas1 == "MAS" || mas1 == "SAM";
            let mas2_match = mas2 == "MAS" || mas2 == "SAM";

            if mas1_match && mas2_match {
                count += 1;
            }
        }
    }

    return count;
}
