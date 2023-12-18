use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day01_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = get_calibration_value(&input, &filter_calibration_value_from_line);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = get_calibration_value(
                        &input,
                        &filter_calibration_value_from_line_from_string,
                    );
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_calibration_value(
    calibration_file: &str,
    f: &dyn Fn(&str) -> Result<u32, String>,
) -> Result<u32, String> {
    calibration_file.lines().map(f).sum()
}

fn filter_calibration_value_from_line(line: &str) -> Result<u32, String> {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    match (digits.first(), digits.last()) {
        (Some(l), Some(r)) => Ok(l * 10 + r),
        _ => Err("Failed to extract digits from line.".to_owned()),
    }
}

fn filter_calibration_value_from_line_from_string(line: &str) -> Result<u32, String> {
    const STRING_TO_DIGIT: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = std::iter::successors(Some(line), |slice: &&str| {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[1..])
        }
    })
    .map(|slice| match slice {
        s if s.len() > 5 => &slice[..5],
        s => s,
    })
    .zip(line.chars())
    .filter_map(|(slice, c)| {
        let digit = c.to_digit(10);
        if digit.is_some() {
            digit.map(Ok)
        } else {
            STRING_TO_DIGIT
                .iter()
                .enumerate()
                .find(|(_, number)| slice.starts_with(*number))
                .map(|(i, _)| u32::try_from(i))
        }
    })
    .collect::<Result<Vec<_>, std::num::TryFromIntError>>()
    .map_err(|err| format!("Failed to find calibration value. '{err}'"))?;
    match (digits.first(), digits.last()) {
        (Some(l), Some(r)) => Ok(l * 10 + r),
        _ => Err("Failed to extract digits from line.".to_owned()),
    }
}

#[cfg(test)]
mod test {
    use crate::get_calibration_value;

    use super::{
        filter_calibration_value_from_line, filter_calibration_value_from_line_from_string,
    };

    const PART1_INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const PART2_INPUT: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1() -> Result<(), String> {
        let day1 = get_calibration_value(PART1_INPUT, &filter_calibration_value_from_line)?;
        assert_eq!(day1, 142);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), String> {
        let day2 =
            get_calibration_value(PART2_INPUT, &filter_calibration_value_from_line_from_string)?;
        assert_eq!(day2, 281);
        Ok(())
    }
}
