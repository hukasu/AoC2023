mod camel_card;

use std::io::Read;

use camel_card::{CamelCardHand, JokerCamelCardHand};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day07_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = calculate_game_winnings::<CamelCardHand>(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = calculate_game_winnings::<JokerCamelCardHand>(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn calculate_game_winnings<'a, T: Ord + From<&'a str>>(input: &'a str) -> Result<u32, String> {
    let mut hands = input
        .lines()
        .map(|line| {
            if let Some((hand, bid)) = line.split_once(' ') {
                let hand: T = hand.into();
                bid.parse::<u32>()
                    .map(|bid| (hand, bid))
                    .map_err(|err| format!("Failed to parse games. '{err}'"))
            } else {
                Err("Line did not contain game and bid.".to_owned())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    hands.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::camel_card::{CamelCardHand, JokerCamelCardHand};

    const PART1_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_test() {
        assert_eq!(
            super::calculate_game_winnings::<CamelCardHand>(PART1_INPUT),
            Ok(6440)
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            super::calculate_game_winnings::<JokerCamelCardHand>(PART1_INPUT),
            Ok(5905)
        );
    }
}
