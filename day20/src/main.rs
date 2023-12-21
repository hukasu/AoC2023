mod modules;

use std::io::Read;

use modules::Network;

const WARMUP_CYCLE: usize = 1000;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day20_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = get_pulse_count(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = turn_rx_on(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_pulse_count(input: &str) -> Result<u64, String> {
    let mut network = Network::try_from(input)?;

    (0..WARMUP_CYCLE)
        .map(|_| network.pulse())
        .try_fold((0, 0), |accum, cur| -> Result<(u64, u64), String> {
            let (low, high, _) = cur;
            Ok((accum.0 + low, accum.1 + high))
        })
        .map(|(low, high)| low * high)
}

fn turn_rx_on(input: &str) -> Result<u64, String> {
    let mut network = Network::try_from(input)?;

    #[allow(clippy::maybe_infinite_iter)]
    (1u64..)
        .find(|_| network.pulse().2)
        .ok_or("Failed to activate rx.".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_INPUT1: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const PART1_INPUT2: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_test() {
        assert_eq!(get_pulse_count(PART1_INPUT1), Ok(32000000));
        assert_eq!(get_pulse_count(PART1_INPUT2), Ok(11687500));
    }
}
