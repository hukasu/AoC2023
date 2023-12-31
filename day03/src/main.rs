use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day03_part1.txt") {
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

#[allow(clippy::too_many_lines)]
fn parts_finder(
    input: &str,
    should_process: &dyn Fn(u8) -> bool,
    symbol_processor: &dyn Fn(&[u32], &mut Vec<u32>),
) -> Result<u32, String> {
    // Searchs for a part number within slice
    // assumes a slice of length 1 or 3
    let search_part_number_in_slice = |slice: &[u8]| -> Option<u32> {
        if slice.len() == 3 && (slice[1] < b'0' || slice[1] > b'9') {
            None
        } else {
            Some(slice.iter().fold(0u32, |accum, d| {
                if d.is_ascii_digit() {
                    accum * 10 + u32::from(d.clamp(&b'0', &b'9').saturating_sub(b'0'))
                } else {
                    accum
                }
            }))
        }
    };
    // Works for any of the combinations were there is a digit touching a symbol from above or below
    // i.e. 123.. .123. ..123 ..*.. ..*.. ..*..
    //      ..*.. ..*.. ..*.. 123.. .123. ..123
    let search_part_number_adjacent_to_symbol =
        |gears: &mut Vec<u32>, slice: &[u8]| -> Result<(), String> {
            if let Some(max) = &slice[1..=5]
                .windows(3)
                .filter_map(search_part_number_in_slice)
                .max()
            {
                gears.push(*max);
            }
            Ok(())
        };
    // Works for any position where a digit is not directly above or below a symbol and to the left
    // i.e. 123. 123* ...*
    //      ...* .... 123.
    let search_part_number_left_diagonal_to_symbol =
        |gears: &mut Vec<u32>, slice: &[u8]| -> Result<(), String> {
            if slice[2].is_ascii_digit() {
                let sub_slice = if slice[1].is_ascii_digit() {
                    &slice[0..=2]
                } else {
                    &slice[2..=2]
                };
                if let Some(left) = search_part_number_in_slice(sub_slice) {
                    gears.push(left);
                } else {
                    Err("Failed to get gear number where there should be one.")?;
                }
            };

            Ok(())
        };
    // Works for any position where a digit is not directly above or below a symbol and to the right
    // i.e. .123 *123 *...
    //      *... .... .123
    let search_part_number_right_diagonal_to_symbol =
        |gears: &mut Vec<u32>, slice: &[u8]| -> Result<(), String> {
            if slice[4].is_ascii_digit() {
                let sub_slice = if slice[5].is_ascii_digit() {
                    &slice[4..=6]
                } else {
                    &slice[4..=4]
                };
                if let Some(right) = search_part_number_in_slice(sub_slice) {
                    gears.push(right);
                } else {
                    Err("Failed to get gear number where there should be one.")?;
                }
            };

            Ok(())
        };

    match input.chars().position(|c| c == '\n') {
        Some(chars_per_line) => {
            // "Empty" line with the same length as the lines from the input
            let all_dots = ".".chars().cycle().take(chars_per_line).collect::<String>();
            // List of lines with "empty" line padding at the beginning and end
            let lines_with_padding = [all_dots.as_str()]
                .into_iter()
                .chain(input.split_whitespace())
                .chain([all_dots.as_str()])
                .map(str::as_bytes)
                .collect::<Vec<_>>();

            lines_with_padding
                .windows(3)
                .try_fold(vec![], |mut parts: Vec<u32>, cur| {
                    if let [top, mid, bottom] = cur {
                        // Possible because input does not have symbols close to the start or end of the lines
                        for ((window_on_top_line, window_on_mid_line), window_on_bottom_line) in
                            top.windows(7).zip(mid.windows(7)).zip(bottom.windows(7))
                        {
                            if should_process(window_on_mid_line[3]) {
                                let mut adjacent_parts = vec![];
                                // Top line tests
                                if window_on_top_line[3].is_ascii_digit() {
                                    search_part_number_adjacent_to_symbol(
                                        &mut adjacent_parts,
                                        window_on_top_line,
                                    )?;
                                } else {
                                    search_part_number_left_diagonal_to_symbol(
                                        &mut adjacent_parts,
                                        window_on_top_line,
                                    )?;
                                    search_part_number_right_diagonal_to_symbol(
                                        &mut adjacent_parts,
                                        window_on_top_line,
                                    )?;
                                }
                                // Mid line tests
                                search_part_number_left_diagonal_to_symbol(
                                    &mut adjacent_parts,
                                    window_on_mid_line,
                                )?;
                                search_part_number_right_diagonal_to_symbol(
                                    &mut adjacent_parts,
                                    window_on_mid_line,
                                )?;
                                // Bottom line tests
                                if window_on_bottom_line[3].is_ascii_digit() {
                                    search_part_number_adjacent_to_symbol(
                                        &mut adjacent_parts,
                                        window_on_bottom_line,
                                    )?;
                                } else {
                                    search_part_number_left_diagonal_to_symbol(
                                        &mut adjacent_parts,
                                        window_on_bottom_line,
                                    )?;
                                    search_part_number_right_diagonal_to_symbol(
                                        &mut adjacent_parts,
                                        window_on_bottom_line,
                                    )?;
                                }

                                symbol_processor(&adjacent_parts, &mut parts);
                            }
                        }

                        Ok(parts)
                    } else {
                        Err("Windows did not have 3 parts.".to_owned())
                    }
                })
                .map(|part_numbers_or_gear_ratios| part_numbers_or_gear_ratios.into_iter().sum())
        }
        None => Err("Input had no lines.".to_owned()),
    }
}

#[cfg(test)]
mod test {
    const PART1_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_test() {
        assert_eq!(super::part_numbers_sum(PART1_INPUT), Ok(4361));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::gear_ratio_sum(PART1_INPUT), Ok(467_835));
    }
}
