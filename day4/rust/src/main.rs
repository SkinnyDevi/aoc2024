use std::fs;

mod wordsearch;

fn main() {
    let contents = fs::read_to_string(
        "C:\\dev\\coding-projects\\advent-of-code\\2024\\day4\\puzzle_input.txt",
    )
    .expect("Should have been able to read the file.");

    // let mut example = "MMMSXXMASM\rMSAMXMSMSA\rAMXSXMAAMM\rMSAMASMSMX\rXMASAMXAMM\rXXAMMXXAMA\rSMSMSASXSS\rSAXAMASAAA\rMAMMMXMMMM\rMXMXAXMASX";
    // example = "..X...\r.SAMX.\r.A..A.\rXMAS.S\r.X....";

    let xmas_count_char = wordsearch::count_xmas(&contents);
    println!("XMAS Count: {xmas_count_char}");

    let x_mas_count = wordsearch::count_x_mas(&contents);
    println!("X-MAS Count: {x_mas_count}");
}
