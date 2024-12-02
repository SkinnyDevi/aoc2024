use std::fs;

mod total_distance;

fn parse_input(file_content: String) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in lines {
        let numbers: Vec<&str> = line.trim().split(" ").collect();
        if line.is_empty() {
            continue;
        }

        let number_left = numbers.first().expect("Could not get first number of line");
        let number_right = numbers.last().expect("Could not get last number of line");

        match number_left.parse::<i32>() {
            Ok(first_int) => left_list.push(first_int),
            Err(e) => println!("Could not parse first int {e}"),
        }

        match number_right.parse::<i32>() {
            Ok(last_int) => right_list.push(last_int),
            Err(e) => println!("Could not parse last int {e}"),
        }
    }

    return (left_list, right_list);
}

fn main() {
    let contents = fs::read_to_string(
        "C:\\dev\\coding-projects\\advent-of-code\\2024\\day1\\puzzle_input.txt",
    )
    .expect("Should have been able to read the file");

    let (left_list, right_list) = parse_input(contents);
    let distance = total_distance::calc_total_distance(left_list.clone(), right_list.clone());
    println!("Distance: {distance}");

    let similarity_score = total_distance::similarity_score(left_list, right_list);
    println!("Score: {similarity_score}");
}
