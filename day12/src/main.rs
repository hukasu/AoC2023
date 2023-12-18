use std::{collections::BTreeMap, io::Read};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day12_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = springs_arrangements(&input, false);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = springs_arrangements(&input, true);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn springs_arrangements(input: &str, unfold: bool) -> Result<u64, String> {
    input
        .lines()
        .zip([unfold].into_iter().cycle())
        .map(|(line, folded)| springs_row_arrangements(line, folded))
        .sum()
}

fn springs_row_arrangements(line: &str, unfold: bool) -> Result<u64, String> {
    if let Some((springs_states, redundance)) = line.split_once(' ') {
        let redundance = redundance
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|err| format!("Failed to read redundant data from line. '{err}'"))?;

        if unfold {
            fit_arrangement(
                &vec![springs_states; 5]
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join("?"),
                vec![redundance; 5]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .as_slice(),
                &mut BTreeMap::new(),
            )
        } else {
            fit_arrangement(springs_states, &redundance, &mut BTreeMap::new())
        }
    } else {
        Err("Failed to split line.".to_owned())
    }
}

fn fit_arrangement<'a>(
    springs_states: &'a str,
    redundance: &'a [usize],
    cache: &mut BTreeMap<(&'a str, &'a [usize]), u64>,
) -> Result<u64, String> {
    if let Some(cached) = cache.get(&(springs_states, redundance)) {
        Ok(*cached)
    } else {
        let arrangements = match redundance {
            [head, tail @ ..] => {
                if springs_states.len() < *head {
                    Ok(0)
                } else {
                    let mut states = springs_states.chars();
                    if (&mut states).take(*head).all(|s| matches!(s, '?' | '#')) {
                        match (springs_states.starts_with('#'), states.next()) {
                            (true, Some('#')) => Ok(0),
                            (false, Some('#')) => {
                                fit_arrangement(&springs_states[1..], redundance, cache)
                            }
                            (true, Some('.' | '?')) => {
                                fit_arrangement(&springs_states[(head + 1)..], tail, cache)
                            }
                            (false, Some('.')) => [
                                if springs_states.chars().take(*head).any(|s| matches!(s, '#')) {
                                    Ok(0)
                                } else {
                                    fit_arrangement(
                                        &springs_states[(head + 1)..],
                                        redundance,
                                        cache,
                                    )
                                },
                                fit_arrangement(&springs_states[(head + 1)..], tail, cache),
                            ]
                            .into_iter()
                            .sum(),
                            (false, Some('?')) => [
                                fit_arrangement(&springs_states[(head + 1)..], tail, cache),
                                fit_arrangement(&springs_states[1..], redundance, cache),
                            ]
                            .into_iter()
                            .sum(),

                            (_, Some(s)) => Err(format!("Invalid state '{s}'.")),
                            (_, None) => {
                                if tail.is_empty() {
                                    Ok(1)
                                } else {
                                    Ok(0)
                                }
                            }
                        }
                    } else if springs_states.starts_with('#') {
                        Ok(0)
                    } else {
                        fit_arrangement(&springs_states[1..], redundance, cache)
                    }
                }
            }
            [] => {
                if springs_states.chars().all(|s| matches!(s, '.' | '?')) {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        };
        if let Ok(arrang) = arrangements {
            cache.insert((springs_states, redundance), arrang);
        }
        arrangements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_test() {
        let lines_res = [1, 4, 1, 1, 4, 10];
        for (line, res) in PART1_INPUT.lines().zip(lines_res) {
            assert_eq!(springs_row_arrangements(line, false), Ok(res));
        }
        assert_eq!(springs_row_arrangements(".??#????.? 2,1", false), Ok(7));
        assert_eq!(springs_row_arrangements(".?#??..?#??.?? 4,1", false), Ok(1));

        assert_eq!(springs_arrangements(PART1_INPUT, false), Ok(21));
    }

    #[test]
    fn part2_test() {
        let lines_res = [1, 16384, 1, 16, 2500, 506_250];
        for (line, res) in PART1_INPUT.lines().zip(lines_res) {
            assert_eq!(springs_row_arrangements(line, true), Ok(res));
        }

        assert_eq!(springs_arrangements(PART1_INPUT, true), Ok(525_152));
    }
}
