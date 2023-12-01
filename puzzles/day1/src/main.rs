use std::fs::File;
use std::io::Read;

fn main() {
    let calibration_lines = read_file("./calibration-doc-example.txt");

    let calibration_values_sum = calibration_values_sum(calibration_lines.clone());
    println!("Calibration sum: {:?}", calibration_values_sum);
}

// Read a text file and return a vector of strings
fn read_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

/// Parse a line of text that should contain at least one digit
/// Return the concatenation of the first and last digit found to form a single 2-digit number.
fn calibration_parse_line(line: String) -> u32 {
    let mut last_digit = 0;
    let mut first_digit = 0;
    let mut first_digit_found = false;

    for c in line.chars() {
        if c.is_digit(10) {
            if !first_digit_found {
                first_digit = c.to_digit(10).unwrap();
                first_digit_found = true;
            }
            // replace the last digit with the current one to get truely the last digit
            last_digit = c.to_digit(10).unwrap();
        }
    }

    first_digit * 10 + last_digit
}

/// Sum all the calibration values
fn calibration_values_sum(calibration_lines: Vec<String>) -> u32 {
    calibration_lines
        .iter()
        .map(|line| calibration_parse_line(line.to_string()))
        .sum()
}

#[cfg(test)]
mod test_calibration_parse_line {
    use super::*;

    #[test]
    fn test_read_file() {
        let calibration_values = read_file("./calibration-doc-example.txt");
        assert_eq!(calibration_values.len(), 4);
    }

    #[test]
    fn test_calibration_parse_line_with_one_digit() {
        let line = String::from("treb7uchet");
        let result = calibration_parse_line(line);
        assert_eq!(result, 77);
    }

    #[test]
    fn test_calibration_parse_line_with_2_digits_first_last() {
        let line = String::from("1abc2");
        let result = calibration_parse_line(line);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_calibration_parse_line_with_2_digits_mixed() {
        let line = String::from("pqr3stu8vwx");
        let result = calibration_parse_line(line);
        assert_eq!(result, 38);
    }

    #[test]
    fn test_calibration_parse_line_with_more_than_2_digits() {
        let line = String::from("a1b2c3d4e5f");
        let result = calibration_parse_line(line);
        assert_eq!(result, 15);
    }
}

#[cfg(test)]
mod test_calibration_values_sum {
    use super::*;

    #[test]
    fn test_calibration_values_sum() {
        let calibration_lines = vec![
            String::from("treb7uchet"),
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
        ];

        let result = calibration_values_sum(calibration_lines);
        assert_eq!(result, 142);
    }
}
