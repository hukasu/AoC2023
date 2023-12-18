use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day09_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = extrapolate_histories_forward(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = extrapolate_histories_backward(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn extrapolate_histories_forward(input: &str) -> Result<i64, String> {
    let histories = input
        .lines()
        .map(process_history)
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    histories
        .iter()
        .map(|his| extrapolate_history_forward(his.as_slice()))
        .sum()
}

fn extrapolate_history_forward(history: &[i64]) -> Result<i64, String> {
    let diffs = std::iter::successors(Some(history.to_vec()), |prev| {
        if prev.iter().all(|diff| diff.eq(&0)) {
            None
        } else {
            Some(prev.windows(2).map(|wind| wind[1] - wind[0]).collect())
        }
    })
    .collect::<Vec<_>>();
    diffs
        .into_iter()
        .rev()
        .try_fold(0, |prev_diff, cur_diffs| match cur_diffs.last() {
            Some(cur_diff) => Ok(prev_diff + cur_diff),
            None => Err("A list of differences was empty.".to_owned()),
        })
}

fn extrapolate_histories_backward(input: &str) -> Result<i64, String> {
    let histories = input
        .lines()
        .map(process_history)
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    histories
        .iter()
        .map(|his| extrapolate_history_backward(his.as_slice()))
        .sum()
}

fn extrapolate_history_backward(history: &[i64]) -> Result<i64, String> {
    let diffs = std::iter::successors(Some(history.to_vec()), |prev| {
        if prev.iter().all(|diff| diff.eq(&0)) {
            None
        } else {
            Some(prev.windows(2).map(|wind| wind[1] - wind[0]).collect())
        }
    })
    .collect::<Vec<_>>();
    diffs
        .into_iter()
        .rev()
        .try_fold(0, |prev_diff, cur_diffs| match cur_diffs.first() {
            Some(cur_diff) => Ok(cur_diff - prev_diff),
            None => Err("A list of differences was empty.".to_owned()),
        })
}

fn process_history(line: &str) -> Result<Vec<i64>, String> {
    line.split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .map_err(|_| format!("Failed to parse history values. '{line}'"))
}

#[cfg(test)]
mod tests {
    const PART1_INPUT1: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_test() {
        assert_eq!(super::extrapolate_histories_forward(PART1_INPUT1), Ok(114));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::extrapolate_histories_backward(PART1_INPUT1), Ok(2));
    }
}
