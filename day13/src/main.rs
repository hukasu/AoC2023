use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day13_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = find_reflections(&input, false);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = find_reflections(&input, true);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn find_reflections(input: &str, search_for_smudge: bool) -> Result<u64, String> {
    input
        .split("\n\n")
        .map(|pattern| find_reflection_in_pattern(pattern, search_for_smudge))
        .sum()
}

fn find_reflection_in_pattern(pattern: &str, search_for_smudge: bool) -> Result<u64, String> {
    if let Some(pattern_value) = find_vertical_reflection(pattern, search_for_smudge)? {
        Ok(pattern_value * 100)
    } else if let Some(pattern_value) = find_horizontal_reflection(pattern, search_for_smudge)? {
        Ok(pattern_value)
    } else {
        Err(format!("Pattern did not have reflection.\n{pattern}"))
    }
}

fn find_vertical_reflection(pattern: &str, search_for_smudge: bool) -> Result<Option<u64>, String> {
    let lines = pattern.lines().collect::<Vec<_>>();

    let search_with_smudge = |i: &usize| -> bool {
        {
            let left = &lines[..*i];
            let right = &lines[*i..];
            left.iter()
                .rev()
                .zip(right)
                .map(|(l, r)| l.chars().zip(r.chars()).filter(|(l, r)| l.ne(r)).count())
                .sum::<usize>()
                .eq(&1usize)
        }
    };
    let search_clean = |i: &usize| -> bool {
        {
            let left = &lines[..*i];
            let right = &lines[*i..];
            left.iter().rev().zip(right).all(|(l, r)| l.eq(r))
        }
    };

    let find_closure: Box<&dyn Fn(&usize) -> bool> = if search_for_smudge {
        Box::new(&search_with_smudge)
    } else {
        Box::new(&search_clean)
    };

    if let Some(reflection_pos) = (1..lines.len()).find(*find_closure) {
        u64::try_from(reflection_pos)
            .map(Some)
            .map_err(|err| err.to_string())
    } else {
        Ok(None)
    }
}

fn find_horizontal_reflection(
    pattern: &str,
    search_for_smudge: bool,
) -> Result<Option<u64>, String> {
    let line_len = pattern.lines().next().ok_or("Pattern was empty.")?.len();
    let transpose = (0..line_len)
        .map(|skip| {
            pattern
                .lines()
                .flat_map(|line| line.chars())
                .skip(skip)
                .step_by(line_len)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    find_vertical_reflection(&transpose, search_for_smudge)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_test() {
        assert_eq!(find_reflections(PART1_INPUT, false), Ok(405));
    }

    #[test]
    fn part2_test() {
        assert_eq!(find_reflections(PART1_INPUT, true), Ok(400));
    }
}
