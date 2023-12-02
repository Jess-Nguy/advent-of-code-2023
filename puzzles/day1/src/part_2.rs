/// Parse a line of text that should contain at least one digit.
/// a digit is a number between 0 and 9 or one of the following words: one, two, three, four, five, six, seven, eight, and nine
/// Return the concatenation of the first and last digit found to form a single 2-digit number.
fn calibration_parse_line_with_word_num(line: String) -> u32 {
    let mut last_digit = 0;
    let mut first_digit = 0;
    let mut first_digit_found = false;
    let mut word_num_start_chars = ['o', 't', 'f', 's', 'e', 'n'];

    for c in line.chars() {
        // check if the char is a digit
        // if not, check if the char is one of the word_num_start_chars
        // if so, loop check the next following chars to see if they match the word_num
    }

    first_digit * 10 + last_digit
}

/// Sum all the calibration values
pub fn calibration_values_sum_with_word_num(calibration_lines: Vec<String>) -> u32 {
    calibration_lines
        .iter()
        .map(|line| calibration_parse_line_with_word_num(line.to_string()))
        .sum()
}

#[cfg(test)]
mod test_calibration_parse_line {
    use super::*;

    #[test]
    fn test_line_with_one_digit_two_word_num() {
        let line = String::from("two1nine");
        let result = calibration_parse_line(line);
        assert_eq!(result, 29);
    }
    #[test]
    fn test_line_with_only_word_num() {
        let line = String::from("eightwothree");
        let result = calibration_parse_line(line);
        assert_eq!(result, 83);
    }
    #[test]
    fn test_line_with_one_digit_two_word_num_mix_char() {
        let line = String::from("abcone2threexyz");
        let result = calibration_parse_line(line);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_line_with_only_word_num() {
        let line = String::from("xtwone3four");
        let result = calibration_parse_line(line);
        assert_eq!(result, 24);
    }
    #[test]
    fn test_line_with_only_word_num() {
        let line = String::from("4nineeightseven2");
        let result = calibration_parse_line(line);
        assert_eq!(result, 42);
    }
    #[test]
    fn test_line_with_only_word_num() {
        let line = String::from("zoneight234");
        let result = calibration_parse_line(line);
        assert_eq!(result, 14);
    }

    /// sixteen is 16 but only care about "one, two, three, four, five, six, seven, eight, and nine"
    #[test]
    fn test_line_with_only_word_num() {
        let line = String::from("7pqrstsixteen");
        let result = calibration_parse_line(line);
        assert_eq!(result, 76);
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
