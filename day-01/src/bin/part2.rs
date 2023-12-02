fn main() {
    let file: &str = include_str!("./input1.txt");
    let result = part2(file);
    println!("{}", result);
}

fn is_number(number_as_str: &str) -> u32 {
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for &(word, value) in &number_words {
        if number_as_str.contains(word) {
            return value;
        }
    }

    0
}

fn check_for_number_reverse(line: &str, mut number_as_str: String, mut two_digit_number: u32) -> u32 {
    for c_rev in line.chars().rev() {
        if c_rev.is_digit(10) {
            two_digit_number = two_digit_number + c_rev.to_digit(10).unwrap();
            break;
        } else {
            number_as_str.insert(0, c_rev);
            let check_for_number = is_number(&number_as_str);
            if check_for_number != 0 {
                number_as_str.clear();
                two_digit_number = two_digit_number + check_for_number;
                break;
            }
        }
    }
    two_digit_number
}

fn part2(file: &str) -> u32 {
    let mut calibration_value: u32 = 0;
    let mut number_as_str: String = String::new();
    for line in file.lines() {
        let mut two_digit_number: u32 = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                two_digit_number = c.to_digit(10).unwrap() * 10;
                number_as_str.clear();
                calibration_value = calibration_value + check_for_number_reverse(line, number_as_str.clone(), two_digit_number);
                break;
            } else {
                number_as_str.push(c);
                let check_for_number = is_number(&number_as_str);
                if check_for_number != 0 {
                    number_as_str.clear();
                    two_digit_number = check_for_number * 10;
                    calibration_value = calibration_value + check_for_number_reverse(line, number_as_str.clone(), two_digit_number);
                    break;
                }
            }
        }
        println!("{}", two_digit_number);
    }
    calibration_value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
