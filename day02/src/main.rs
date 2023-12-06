use std::io::Read;

#[derive(Debug, Default, PartialEq)]
struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl std::iter::Sum for Game {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Game::default(),
            |Game { red, green, blue },
             Game {
                 red: red_cur,
                 green: green_cur,
                 blue: blue_cur,
             }| {
                Game {
                    red: red + red_cur,
                    green: green + green_cur,
                    blue: blue + blue_cur,
                }
            },
        )
    }
}

const GAME_CUTOFF: Game = Game {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day2_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = valid_games(&input, &GAME_CUTOFF);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = games_power(&input);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

/// Tests all Games and sum the IDs of the valid one
fn valid_games(input: &str, cutoff: &Game) -> Result<u32, String> {
    input.lines().map(|line| valid_game(line, cutoff)).sum()
}

/// Reads a Game and returns its ID if valid
fn valid_game(line: &str, cutoff: &Game) -> Result<u32, String> {
    match line.split(':').collect::<Vec<_>>().as_slice() {
        [game_id, game] => match game_id.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Game", d] => match d.parse::<u32>() {
                Ok(id) => match verify_game(game, cutoff) {
                    Ok(test) => {
                        if test {
                            Ok(id)
                        } else {
                            // Using zero as it is a neutral number on sum
                            Ok(0)
                        }
                    }
                    Err(err) => Err(format!("Failed to parse game {id}. '{err}'")),
                },
                Err(err) => Err(format!("Failed to parse game id. '{err}'")),
            },
            _ => Err(format!("Game header is malformed. '{game_id}'")),
        },
        _ => Err(format!("Game is malformed. '{line}'")),
    }
}

/// Verify if all sets in Game are within cutoff
fn verify_game(
    game: &str,
    Game {
        red: red_cutoff,
        green: green_cutoff,
        blue: blue_cutoff,
    }: &Game,
) -> Result<bool, String> {
    Ok(game
        .split(';')
        .map(parse_set)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .all(|Game { red, green, blue }| {
            &red <= red_cutoff && &green <= green_cutoff && &blue <= blue_cutoff
        }))
}

/// Parses a set into a [Game]
fn parse_set(set: &str) -> Result<Game, String> {
    set.split(',').try_fold(
        Game::default(),
        |Game { red, green, blue }, set_piece| match set_piece
            .split_whitespace()
            .collect::<Vec<_>>()
            .as_slice()
        {
            [d, "red"] => Ok(Game {
                red: red
                    + d.parse::<u32>()
                        .map_err(|err| format!("Failed to parse red digit from set. '{err}'"))?,
                green,
                blue,
            }),
            [d, "green"] => Ok(Game {
                red,
                green: green
                    + d.parse::<u32>()
                        .map_err(|err| format!("Failed to parse green digit from set. '{err}'"))?,
                blue,
            }),
            [d, "blue"] => Ok(Game {
                red,
                green,
                blue: blue
                    + d.parse::<u32>()
                        .map_err(|err| format!("Failed to parse blue digit from set. '{err}'"))?,
            }),
            _ => Err("Failed to parse set.".to_owned()),
        },
    )
}

/// Sum the power of all games
fn games_power(input: &str) -> Result<u32, String> {
    input.lines().map(game_power).sum()
}

/// Get the power of a game
fn game_power(line: &str) -> Result<u32, String> {
    match line.split(':').collect::<Vec<_>>().as_slice() {
        [game_id, game] => match game_id.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Game", d] => match d.parse::<u32>() {
                Ok(id) => match min_viable_set(game) {
                    Ok(Game { red, green, blue }) => Ok(red * green * blue),
                    Err(err) => Err(format!("Failed to parse game {id}. '{err}'")),
                },
                Err(err) => Err(format!("Failed to parse game id. '{err}'")),
            },
            _ => Err(format!("Game header is malformed. '{game_id}'")),
        },
        _ => Err(format!("Game is malformed. '{line}'")),
    }
}

/// Get the minimum viable set for a game
fn min_viable_set(game: &str) -> Result<Game, String> {
    game.split(';').map(parse_set).try_fold(
        Game::default(),
        |Game {
             red: red_accum,
             green: green_accum,
             blue: blue_accum,
         },
         cur| {
            if let Ok(Game { red, green, blue }) = cur {
                Ok(Game {
                    red: red_accum.max(red),
                    green: green_accum.max(green),
                    blue: blue_accum.max(blue),
                })
            } else {
                cur
            }
        },
    )
}

#[cfg(test)]
mod test {
    const PART1_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn parse_set_test() {
        assert_eq!(
            super::parse_set("3 blue, 4 red"),
            Ok(crate::Game {
                red: 4,
                green: 0,
                blue: 3
            })
        );
        assert_eq!(
            super::parse_set("1 red, 2 green, 6 blue"),
            Ok(crate::Game {
                red: 1,
                green: 2,
                blue: 6
            })
        );
        assert_eq!(
            super::parse_set("2 green"),
            Ok(crate::Game {
                red: 0,
                green: 2,
                blue: 0
            })
        );
    }

    #[test]
    fn parse_game_test() {
        assert_eq!(
            super::verify_game(
                "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                &super::GAME_CUTOFF
            ),
            Ok(true)
        );
        assert_eq!(
            super::verify_game("10 red, 7 green, 3 blue; 5 blue, 3 red, 10 green; 4 blue, 14 green, 7 red; 1 red, 11 green; 6 blue, 17 green, 15 red; 18 green, 7 red, 5 blue", &super::GAME_CUTOFF),
            Ok(false)
        );
    }

    #[test]
    fn valid_game_test() {
        assert_eq!(
            super::valid_game(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                &super::GAME_CUTOFF
            ),
            Ok(1)
        );
        assert_eq!(
            super::valid_game("Game 1: 10 red, 7 green, 3 blue; 5 blue, 3 red, 10 green; 4 blue, 14 green, 7 red; 1 red, 11 green; 6 blue, 17 green, 15 red; 18 green, 7 red, 5 blue", &super::GAME_CUTOFF),
            Ok(0)
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(super::valid_games(PART1_INPUT, &super::GAME_CUTOFF), Ok(8))
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::games_power(PART1_INPUT), Ok(2286))
    }
}
