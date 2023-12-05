use std::{ops::Range, str::Lines};

type MappingSection = Vec<(Range<u64>, Range<u64>)>;

#[derive(Debug)]
pub struct SeedMapper {
    seeds: Vec<u64>,
    seed_to_soil: MappingSection,
    soil_to_fertilizer: MappingSection,
    fertilizer_to_water: MappingSection,
    water_to_light: MappingSection,
    light_to_temperature: MappingSection,
    temperature_to_humidity: MappingSection,
    humidity_to_location: MappingSection,
}

impl SeedMapper {
    pub fn get_lowest_seed_location(&self, seed_ranges: bool) -> Result<u64, String> {
        self.get_seed_locations(seed_ranges)
            .min()
            .ok_or("Failed to map seeds to the min location.".to_owned())
    }

    fn get_seed_locations<'a>(&'a self, seed_ranges: bool) -> Box<dyn Iterator<Item = u64> + 'a> {
        if seed_ranges {
            // takes pairs of values from 'seeds'
            // the first is the start of a range
            // the second the length of the range
            self.map_seed_ranges_to_locations()
        } else {
            // take values from 'seeds' as is
            self.map_seeds_to_locations(Box::new(self.seeds.iter().copied()))
        }
    }

    fn map_item_on_section(item: u64, mapping: &[(Range<u64>, Range<u64>)]) -> u64 {
        mapping
            .iter()
            .find_map(|(source, dest)| {
                if source.contains(&item) {
                    Some(dest.start + (item - source.start))
                } else {
                    None
                }
            })
            .unwrap_or(item)
    }

    fn map_seed_ranges_to_locations<'a>(&'a self) -> Box<dyn Iterator<Item = u64> + 'a> {
        let seed_ranges = self.seeds.chunks(2).map(|windows| {
            if let [start, range] = windows {
                *start..(start + range)
            } else {
                panic!("Chunk did not have 2 items.")
            }
        });
        Box::new(seed_ranges.flat_map(|range| {
            let mut res = vec![];
            let mut processing_ranges = vec![range];
            while let Some(range) = processing_ranges.pop() {
                let mappings: Vec<u64> = self
                    .map_seeds_to_locations(Box::new([range.start, range.end - 1].into_iter()))
                    .collect();
                if let [mapped_start, mapped_end] = mappings.as_slice() {
                    match mapped_start.cmp(mapped_end) {
                        std::cmp::Ordering::Greater => {
                            let mid = range.start + (range.end - range.start) / 2;
                            processing_ranges.push(range.start..mid);
                            processing_ranges.push(mid..range.end);
                        }
                        std::cmp::Ordering::Equal => {
                            res.push(*mapped_start);
                        }
                        std::cmp::Ordering::Less => {
                            if mapped_end - mapped_start == (range.end - 1) - range.start {
                                res.push(*mapped_start);
                            } else {
                                let mid = range.start + (range.end - range.start) / 2;
                                processing_ranges.push(range.start..mid);
                                processing_ranges.push(mid..range.end);
                            }
                        }
                    }
                }
            }
            res
        }))
    }

    fn map_seeds_to_locations<'a>(
        &'a self,
        seeds: Box<dyn Iterator<Item = u64> + 'a>,
    ) -> Box<dyn Iterator<Item = u64> + 'a> {
        Box::new(
            seeds
                .map(|seed| Self::map_item_on_section(seed, &self.seed_to_soil))
                .map(|soil| Self::map_item_on_section(soil, &self.soil_to_fertilizer))
                .map(|fertilizer| Self::map_item_on_section(fertilizer, &self.fertilizer_to_water))
                .map(|water| Self::map_item_on_section(water, &self.water_to_light))
                .map(|light| Self::map_item_on_section(light, &self.light_to_temperature))
                .map(|temperature| {
                    Self::map_item_on_section(temperature, &self.temperature_to_humidity)
                })
                .map(|humidity| Self::map_item_on_section(humidity, &self.humidity_to_location)),
        )
    }

    fn read_almanac_section(
        section_title: &str,
        lines: &mut Lines,
    ) -> Result<MappingSection, String> {
        let header = lines
            .next()
            .ok_or(format!("EOF before start of '{section_title}' section."))?;
        if header.ne(section_title) {
            Err(format!(
                "Section header is wrong. Expected: '{section_title}', Got: '{header}'"
            ))?;
        }
        let mut res = vec![];

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let ranges_start_and_len = line
                .split_whitespace()
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()
                .map_err(|err| format!("Failed to read ranges start and len. '{err}'"))?;
            match ranges_start_and_len.as_slice() {
                [dest_start, source_start, len] => res.push((
                    *source_start..(source_start + len),
                    *dest_start..(dest_start + len),
                )),
                a => Err(format!("Line had wrong number of items. '{a:?}'"))?,
            }
        }

        Ok(res)
    }
}

impl TryFrom<&str> for SeedMapper {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines = input.lines();

        let seeds = {
            let first_line = lines.next().ok_or("Failed to read first line.")?;
            let split = first_line
                .split_once(':')
                .ok_or("Failed to split first line.")?;
            if split.0.ne("seeds") {
                Err("First line does not contain seeds.")?
            } else {
                split
                    .1
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<u64>, _>>()
                    .map_err(|err| format!("Failed to read seeds. '{err}'"))?
            }
        };

        let spacing = lines.next().ok_or("Input only had one line.")?;
        if !spacing.is_empty() {
            Err("There was no spacing between 'seeds' and 'seed-to-soil' sections.")?
        }

        let seed_to_soil = Self::read_almanac_section("seed-to-soil map:", &mut lines)?;
        let soil_to_fertilizer = Self::read_almanac_section("soil-to-fertilizer map:", &mut lines)?;
        let fertilizer_to_water =
            Self::read_almanac_section("fertilizer-to-water map:", &mut lines)?;
        let water_to_light = Self::read_almanac_section("water-to-light map:", &mut lines)?;
        let light_to_temperature =
            Self::read_almanac_section("light-to-temperature map:", &mut lines)?;
        let temperature_to_humidity =
            Self::read_almanac_section("temperature-to-humidity map:", &mut lines)?;
        let humidity_to_location =
            Self::read_almanac_section("humidity-to-location map:", &mut lines)?;

        Ok(SeedMapper {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}
