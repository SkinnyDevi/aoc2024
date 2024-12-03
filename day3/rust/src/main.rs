use std::fs;

mod memory_corrector;

fn main() {
    let contents = fs::read_to_string(
        "C:\\dev\\coding-projects\\advent-of-code\\2024\\day3\\puzzle_input.txt",
    )
    .expect("Should have been able to read the file");

    // let example = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    let corrected_multiplication_output: i32 = memory_corrector::multiply_memory(&contents, None);
    println!("Corrected calculation: {corrected_multiplication_output}");

    // let example = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    let conditional_correcter_output = memory_corrector::multiply_memory(&contents, Some(true));
    println!("(Conditional) Corrected calculation: {conditional_correcter_output}");
}
