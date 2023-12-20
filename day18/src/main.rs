use std::{io::Read, ops::RangeInclusive};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day18_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = dig_trench(&input, false);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = dig_trench(&input, true);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dig_trench(input: &str, apply_fix: bool) -> Result<u64, String> {
    let instructions = if apply_fix {
        get_fixed_instructions(input)
    } else {
        get_instructions(input)
    }?;

    let bb = trench_bounding_box(&instructions)?;
    let max_depth = isize::try_from(bb.1.count())
        .map_err(|err| format!("Failed to calculate max depth. '{err}'"))?
        + 1;

    let lagoon: isize = instructions
        .iter()
        .scan((0isize, 0isize), |prev, instruction| match instruction {
            (Direction::Right, meters) => {
                let start = std::mem::replace(prev, (prev.0 + meters, prev.1));
                let area = (1, *meters, max_depth - start.1);
                Some(area)
            }
            (Direction::Down, meters) => {
                let _ = std::mem::replace(prev, (prev.0, prev.1 + meters));
                let area = (1, 1, *meters);
                Some(area)
            }
            (Direction::Left, meters) => {
                let start = std::mem::replace(prev, (prev.0 - meters, prev.1));
                let area = (-1, *meters, (max_depth - 1) - start.1);
                Some(area)
            }
            (Direction::Up, meters) => {
                let _ = std::mem::replace(prev, (prev.0, prev.1 - meters));
                let area = (0, 0, 0);
                Some(area)
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(multiplier, x, y)| multiplier * (x * y))
        .sum::<isize>()
        + 1isize;

    u64::try_from(lagoon).map_err(|err| format!("Failed to calculate lagoon volume. '{err}'"))
}

fn trench_bounding_box(
    mut instructions: &[(Direction, isize)],
) -> Result<(RangeInclusive<isize>, RangeInclusive<isize>), String> {
    let trench = std::iter::successors(Some((0isize, 0isize)), |prev| {
        if !instructions.is_empty() {
            let head = instructions[0];
            instructions = &instructions[1..];
            match head {
                (Direction::Right, meters) => Some((prev.0 + meters, prev.1)),
                (Direction::Down, meters) => Some((prev.0, prev.1 + meters)),
                (Direction::Left, meters) => Some((prev.0 - meters, prev.1)),
                (Direction::Up, meters) => Some((prev.0, prev.1 - meters)),
            }
        } else {
            None
        }
    })
    .collect::<Vec<_>>();

    let min_x = trench
        .iter()
        .min_by_key(|dug| dug.0)
        .ok_or("Could not find leftmost X.")?
        .0;
    let max_x = trench
        .iter()
        .max_by_key(|dug| dug.0)
        .ok_or("Could not find rightmost X.")?
        .0;
    let min_y = trench
        .iter()
        .min_by_key(|dug| dug.1)
        .ok_or("Could not find upmost Y.")?
        .1;
    let max_y = trench
        .iter()
        .max_by_key(|dug| dug.1)
        .ok_or("Could not find downmost Y.")?
        .1;

    Ok((min_x..=max_x, min_y..=max_y))
}

fn get_fixed_instructions(input: &str) -> Result<Vec<(Direction, isize)>, String> {
    input
        .lines()
        .map(|line| -> Result<(Direction, isize), String> {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                [_, _, color] => {
                    let (meters_hex, dir_digit) = color
                        .trim_start_matches("(#")
                        .trim_end_matches(')')
                        .split_at(5);
                    let dir = match dir_digit {
                        "0" => Direction::Right,
                        "1" => Direction::Down,
                        "2" => Direction::Left,
                        "3" => Direction::Up,
                        _ => Err(format!("Failed to parse direction from hex. '{color}'"))?,
                    };
                    let meters = isize::from_str_radix(meters_hex, 16)
                        .map_err(|err| format!("Failed to parse meters from hex. '{err}'"))?;
                    Ok((dir, meters))
                }
                _ => Err(format!("Malformed instruction. '{line}'"))?,
            }
        })
        .collect()
}

fn get_instructions(input: &str) -> Result<Vec<(Direction, isize)>, String> {
    input
        .lines()
        .map(|line| -> Result<(Direction, isize), String> {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                [dir_code, count, _] => {
                    let dir = match *dir_code {
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "U" => Direction::Up,
                        _ => Err(format!("Unknown direction. '{dir_code}'"))?,
                    };
                    let meters = count
                        .parse::<isize>()
                        .map_err(|err| format!("Failed to parse meters from hex. '{err}'"))?;
                    Ok((dir, meters))
                }
                _ => Err(format!("Malformed instruction. '{line}'"))?,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_test() {
        assert_eq!(dig_trench(PART1_INPUT, false), Ok(62));
    }

    #[test]
    fn part2_test() {
        assert_eq!(dig_trench(PART1_INPUT, true), Ok(952_408_144_115));
    }
}
