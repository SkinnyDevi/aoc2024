use std::fs;

mod report_processing;

fn parse_input(file_content: String) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = file_content.split("\n").collect();

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let levels_str: Vec<&str> = line.trim().split(" ").collect();
        let mut levels: Vec<i32> = Vec::new();
        for level_str in levels_str {
            match level_str.parse::<i32>() {
                Ok(level) => levels.push(level),
                Err(e) => println!("Could not parse level ({level_str}): {e}"),
            }
        }

        reports.push(levels);
    }

    return reports;
}

fn main() {
    let contents = fs::read_to_string(
        "C:\\dev\\coding-projects\\advent-of-code\\2024\\day2\\puzzle_input.txt",
    )
    .expect("Should have been able to read the file");

    let reports = parse_input(contents);
    let safe_reports: i32 = report_processing::get_safe_reports_amount(&reports, None);
    println!("Safe reports: {safe_reports}");

    let dampener_safe_reports = report_processing::get_safe_reports_amount(&reports, Some(true));
    println!("(Dampened) Safe reports: {dampener_safe_reports}");
}
