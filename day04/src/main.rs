use std::{collections::BTreeSet, io::Read};

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: BTreeSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn count_winning_numbers(&self) -> Result<u32, String> {
        u32::try_from(
            self.numbers
                .iter()
                .filter(|number| self.winning_numbers.contains(number))
                .count(),
        )
        .map_err(|err| format!("Failed to count winning cards. '{err}'"))
    }
}

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day04_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = card_pile_worth(&input);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = count_card_pile(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn card_pile_worth(input: &str) -> Result<u32, String> {
    input.lines().map(card_worth).sum()
}

fn card_worth(card: &str) -> Result<u32, String> {
    let card = read_card(card)?;
    Ok(2u32.pow(card.count_winning_numbers()?) / 2)
}

fn count_card_pile(input: &str) -> Result<u32, String> {
    let cards = input
        .lines()
        .map(read_card)
        .collect::<Result<Vec<_>, _>>()?;
    let mut card_count = vec![1; cards.len()];

    for card in cards {
        let copies_of_cur_card = card_count[card.id - 1];
        let winning_count = card.count_winning_numbers()? as usize;
        card_count[card.id..(card.id + winning_count)]
            .iter_mut()
            .for_each(|cur| *cur += copies_of_cur_card);
    }

    Ok(card_count.into_iter().sum())
}

fn read_card(card: &str) -> Result<Card, String> {
    if let Some((card_id, card_contents)) = card.split_once(':') {
        let id = {
            let id_str = card_id
                .split_once(' ')
                .ok_or("Failed to split card header.")?;
            id_str
                .1
                .trim()
                .parse()
                .map_err(|err| format!("Failed to parse card id. '{err}'"))?
        };
        let (winning_numbers, card_numbers) = {
            if let Some((winning, numbers)) = card_contents.split_once('|') {
                Ok((
                    winning
                        .split_whitespace()
                        .filter(|str| !str.is_empty())
                        .map(|num| {
                            num.parse::<u32>().map_err(|err| {
                                format!("Failed to parse card winning numbers. '{err}'")
                            })
                        })
                        .collect::<Result<BTreeSet<u32>, String>>()?,
                    numbers
                        .split_whitespace()
                        .filter(|str| !str.is_empty())
                        .map(|num| {
                            num.parse::<u32>()
                                .map_err(|err| format!("Failed to parse card numbers. '{err}'"))
                        })
                        .collect::<Result<Vec<u32>, String>>()?,
                ))
            } else {
                Err("Failed to parse card contents.".to_owned())
            }
        }?;
        Ok(Card {
            id,
            winning_numbers,
            numbers: card_numbers,
        })
    } else {
        Err("Could not get card id.".to_owned())
    }
}

#[cfg(test)]
mod test {
    const PART1_INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_test() {
        assert_eq!(super::card_pile_worth(PART1_INPUT), Ok(13));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::count_card_pile(PART1_INPUT), Ok(30));
    }
}
