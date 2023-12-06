use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day06_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = ways_to_beat_race_records(&input, false);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = ways_to_beat_race_records(&input, true);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn ways_to_beat_race_records(input: &str, single_race: bool) -> Result<u64, String> {
    let to_vec = |input: &str| -> Result<Vec<u64>, String> {
        input
            .split_whitespace()
            .skip(1)
            .map(|time| time.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|err| format!("Failed to parse races information. '{err}'"))
    };

    let parse_bad_kerning = |input: &str| -> Result<u64, String> {
        input
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse::<u64>()
            .map_err(|err| format!("Failed to parse races information with bad kerning. '{err}'"))
    };

    let mut lines = input.lines();
    let race_duration = {
        let first_line = lines.next().ok_or("Could not read times line.")?;
        if single_race {
            vec![parse_bad_kerning(first_line)?]
        } else {
            to_vec(first_line)?
        }
    };
    let race_record = {
        let second_line = lines.next().ok_or("Could not read times line.")?;
        if single_race {
            vec![parse_bad_kerning(second_line)?]
        } else {
            to_vec(second_line)?
        }
    };

    Ok(race_duration
        .into_iter()
        .zip(race_record)
        .map(|(race_duration, race_record)| {
            if let Some(first_win) = (0..race_duration)
                .find(|boat_charge| (boat_charge * (race_duration - boat_charge)) > race_record)
            {
                race_duration - (first_win * 2) + 1
            } else {
                0
            }
        })
        .product())
}

#[cfg(test)]
mod tests {
    const PART1_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn part1_test() {
        assert_eq!(
            super::ways_to_beat_race_records(PART1_INPUT, false),
            Ok(288)
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            super::ways_to_beat_race_records(PART1_INPUT, true),
            Ok(71503)
        );
    }
}
