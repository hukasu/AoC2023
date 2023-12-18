mod mapper;

use std::io::Read;

fn main() -> Result<(), String> {
    match std::fs::File::open("inputs/day05_part1.txt") {
        Ok(mut file) => {
            let mut input = String::new();
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    let timer = std::time::Instant::now();
                    let part1 = get_lowest_seed_location(&input, false);
                    println!("{:?}: {part1:?}", timer.elapsed());

                    let timer = std::time::Instant::now();
                    let part2 = get_lowest_seed_location(&input, true);
                    println!("{:?}: {part2:?}", timer.elapsed());

                    Ok(())
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_lowest_seed_location(input: &str, seed_ranges: bool) -> Result<u64, String> {
    let almanac: mapper::SeedMapper = input.try_into()?;
    almanac.get_lowest_seed_location(seed_ranges)
}

#[cfg(test)]
mod test {
    const PART1_INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_test() {
        assert_eq!(super::get_lowest_seed_location(PART1_INPUT, false), Ok(35));
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::get_lowest_seed_location(PART1_INPUT, true), Ok(46));
    }
}
