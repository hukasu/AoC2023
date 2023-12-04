use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day3_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = part_numbers_sum(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = gear_ratio_sum(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn part_numbers_sum(input: &str) -> Result<u32, String> {
    let should_process = |digit: u8| -> bool { !digit.is_ascii_digit() && digit != b'.' };
    let processor = |found_parts: &[u32], accumulator: &mut Vec<u32>| {
        accumulator.push(found_parts.iter().sum());
    };
    parts_finder(input, &should_process, &processor)
}

fn gear_ratio_sum(input: &str) -> Result<u32, String> {
    let should_process = |digit: u8| -> bool { digit == b'*' };
    let processor = |found_parts: &[u32], accumulator: &mut Vec<u32>| {
        if found_parts.len() == 2 {
            accumulator.push(found_parts.iter().product());
        }
    };
    parts_finder(input, &should_process, &processor)
}

fn parts_finder(
    input: &str,
    should_process: &dyn Fn(u8) -> bool,
    symbol_processor: &dyn Fn(&[u32], &mut Vec<u32>),
) -> Result<u32, String> {
    let line_to_gears = |window: &[u8]| -> Option<u32> {
        if window.len() > 1 && (window[1] < b'0' || window[1] > b'9') {
            None
        } else {
            Some(window.iter().fold(0u32, |accum, d| {
                if d.is_ascii_digit() {
                    accum * 10 + d.clamp(&b'0', &b'9').saturating_sub(b'0') as u32
                } else {
                    accum
                }
            }))
        }
    };
    let adjacent = |gears: &mut Vec<u32>, slice: &[u8]| -> Result<(), String> {
        if let Some(max) = &slice[1..=5].windows(3).filter_map(line_to_gears).max() {
            gears.push(*max);
        }
        Ok(())
    };
    let left_diagonal = |gears: &mut Vec<u32>, slice: &[u8]| -> Result<(), String> {
        if slice[2].is_ascii_digit() {
            let sub_slice = if slice[1].is_ascii_digit() {
                &slice[0..=2]
            } else {
                &slice[2..=2]
            };
            if let Some(left) = line_to_gears(sub_slice) {
                gears.push(left);
            } else {
                Err("Failed to get gear number where there should be one.")?
            }
        };

        Ok(())
    };
    let right_diagonal = |gears: &mut Vec<u32>, slice: &[u8]| -> Result<(), String> {
        if slice[4].is_ascii_digit() {
            let sub_slice = if slice[5].is_ascii_digit() {
                &slice[4..=6]
            } else {
                &slice[4..=4]
            };
            if let Some(right) = line_to_gears(sub_slice) {
                gears.push(right);
            } else {
                Err("Failed to get gear number where there should be one.")?
            }
        };

        Ok(())
    };

    match input.chars().position(|c| c == '\n') {
        Some(chars_per_line) => {
            let all_dots = String::from_iter(".".chars().cycle().take(chars_per_line));
            let lines = [all_dots.as_str()]
                .into_iter()
                .chain(input.split_whitespace())
                .chain([all_dots.as_str()])
                .map(|s| s.as_bytes())
                .collect::<Vec<_>>();
            lines
                .windows(3)
                .try_fold(vec![], |mut parts: Vec<u32>, cur| {
                    if let [top, mid, bottom] = cur {
                        // Possible because input does not have '*' close to the start or end of the lines
                        for ((a, b), c) in top.windows(7).zip(mid.windows(7)).zip(bottom.windows(7))
                        {
                            if should_process(b[3]) {
                                let mut adjacent_parts = vec![];
                                // Top line tests
                                if a[3].is_ascii_digit() {
                                    adjacent(&mut adjacent_parts, a)?;
                                } else {
                                    left_diagonal(&mut adjacent_parts, a)?;
                                    right_diagonal(&mut adjacent_parts, a)?;
                                }
                                // Mid line tests
                                left_diagonal(&mut adjacent_parts, b)?;
                                right_diagonal(&mut adjacent_parts, b)?;
                                // Bottom line tests
                                if c[3].is_ascii_digit() {
                                    adjacent(&mut adjacent_parts, c)?;
                                } else {
                                    left_diagonal(&mut adjacent_parts, c)?;
                                    right_diagonal(&mut adjacent_parts, c)?;
                                }

                                symbol_processor(&adjacent_parts, &mut parts);
                            }
                        }

                        Ok(parts)
                    } else {
                        Err("Windows did not have 3 parts.".to_owned())
                    }
                })
                .map(|ratios| ratios.into_iter().sum())
        }
        None => Err("Input had no lines.".to_owned()),
    }
}

#[cfg(test)]
mod test {
    const PART1_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn part1_test() {
        assert_eq!(super::part_numbers_sum(PART1_INPUT), Ok(4361))
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::gear_ratio_sum(PART1_INPUT), Ok(467835))
    }
}
