fn main() {
    let file: &str = include_str!("./input1.txt");
    let result = part1(file);
    println!("{}", result);
}

fn part1(file: &str) -> u32 {
    let mut calibration_value:u32 = 0;
    for line in file.lines() {
        //line.chars().next().unwrap().is_digit(10)
        let mut two_digit_number: u32 = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                two_digit_number = c.to_digit(10).unwrap()*10;
                for c_rev in line.chars().rev() {
                    if c_rev.is_digit(10) {
                        two_digit_number = two_digit_number+c_rev.to_digit(10).unwrap();
                        calibration_value = calibration_value + two_digit_number;
                        break
                    } 
                }
                break
            }
        }
    }
    calibration_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "1abc2\n\
    pqr3stu8vwx\n\
    a1b2c3d4e5f\n\
    treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }
}
