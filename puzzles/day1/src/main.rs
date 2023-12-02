mod part_1;

use crate::part_1::calibration_values_sum;
use crate::part_2::calibration_values_sum_with_word_num;
use colored::Colorize;
use std::fs::File;
use std::io::Read;

fn main() {
    let calibration_lines = read_file("./puzzle-input.txt");
    // let calibration_lines = read_file("./calibration-doc-example.txt");

    let calibration_values_sum = calibration_values_sum(calibration_lines.clone());
    println!(
        "PART 1 - Calibration sum: {}",
        calibration_values_sum.to_string().blue()
    );
    let calibration_values_sum_with_word_num =
        calibration_values_sum_with_word_num(calibration_lines.clone());
    println!(
        "PART 2 - Calibration sum w/ word numbers: {}",
        calibration_values_sum_with_word_num.to_string().blue()
    );
}

// Read a text file and return a vector of strings
fn read_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

#[cfg(test)]
#[test]
fn test_read_file() {
    let calibration_values = read_file("./calibration-doc-example.txt");
    assert_eq!(calibration_values.len(), 4);
}
