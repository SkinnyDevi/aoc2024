use regex::Regex;

fn wordsearch_to_matrix(wordsearch: &String) -> Vec<Vec<char>> {
    let raw_vec: Vec<&str> = wordsearch.split("\r").collect();

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for row in raw_vec {
        matrix.push(row.chars().collect());
    }

    return matrix;
}

fn count_vertical(wordsearch: &String) -> i32 {
    let matrix = wordsearch_to_matrix(wordsearch);
    let line_len = matrix
        .get(0)
        .expect("Expected a string for len, not found")
        .len();

    let mut count = 0;
    for row in 0..matrix.len() - 3 {
        for col in 0..line_len {
            let mut chars: Vec<char> = Vec::new();
            for i in 0..4 {
                chars.push(matrix[row + i][col]);
            }

            let joined_str: String = chars.iter().collect();
            if joined_str == "XMAS" || joined_str == "SAMX" {
                count += 1;
            }
        }
    }

    return count;
}

fn count_diagonal_right(wordsearch: &String) -> i32 {
    let matrix = wordsearch_to_matrix(wordsearch);
    let line_len = matrix
        .get(0)
        .expect("Expected a string for len, not found")
        .len();

    let mut count = 0;
    for row in 0..matrix.len() - 3 {
        for col in 0..line_len - 3 {
            let mut chars: Vec<char> = Vec::new();
            for i in 0..4 {
                chars.push(matrix[row + i][col + i]);
            }

            let joined_str: String = chars.iter().collect();
            if joined_str == "XMAS" || joined_str == "SAMX" {
                count += 1;
            }
        }
    }

    return count;
}

fn count_diagonal_left(wordsearch: &String) -> i32 {
    let matrix = wordsearch_to_matrix(wordsearch);
    let line_len = matrix
        .get(0)
        .expect("Expected a string for len, not found")
        .len();

    let mut count = 0;
    for row in 0..matrix.len() - 3 {
        for col in (3..line_len).rev() {
            let mut chars: Vec<char> = Vec::new();
            for i in 0..4 {
                chars.push(matrix[row + i][col - i]);
            }

            let joined_str: String = chars.iter().collect();
            // print!("({row}, {col}) - ");
            // print!("{joined_str}");
            if joined_str == "XMAS" || joined_str == "SAMX" {
                // print!(" <- found");
                count += 1;
            }
            // println!("");
        }
    }

    return count;
}

fn count_horizontal(wordsearch: &String) -> i32 {
    let xmas_count = Regex::new(r"XMAS").unwrap().find_iter(wordsearch).count() as i32;
    let samx_count = Regex::new(r"SAMX").unwrap().find_iter(wordsearch).count() as i32;
    return xmas_count + samx_count;
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

fn count_diagonal_right_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
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

fn count_diagonal_left_from_char(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
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

        // diagonal right
        if col < matrix[row].len() - 3 {
            count += count_diagonal_right_from_char(matrix, row, col);
        }

        // diagonal left
        if col >= 3 {
            count += count_diagonal_left_from_char(matrix, row, col);
        }
    }

    return count;
}

pub fn count_xmas_char_matrix(wordsearch: &str) -> i32 {
    let matrix = wordsearch_to_matrix(&String::from(wordsearch));

    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            count += count_from_char(&matrix, i, j);
        }
    }

    return count;
}

pub fn count_xmas(wordsearch: &str) -> i32 {
    let raw_content = String::from(wordsearch);

    let horizontal = count_horizontal(&raw_content);
    let vertical = count_vertical(&raw_content);
    let diagonal_right = count_diagonal_right(&raw_content);
    let diagonal_left = count_diagonal_left(&raw_content);

    println!("{horizontal}, {vertical}, {diagonal_right}, {diagonal_left}");

    return horizontal + vertical + diagonal_right + diagonal_left;
}
