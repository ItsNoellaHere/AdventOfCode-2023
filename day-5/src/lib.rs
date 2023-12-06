#[derive(Debug)]
pub struct Input {
    pub seeds: Vec<usize>,
    pub range_store: Vec<Vec<Range>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Range {
    pub fn map_range(&self, range: Range) -> Option<Range> {
        let start = self.map(range.source_start)?;
        let end = self.map(range.source_start + range.length)?;
        Some(Range::new(start, end - start, range.length))
    }

    pub fn map(&self, value: usize) -> Option<usize> {
        if value < self.source_start {
            return None;
        }
        if value > self.source_start + self.length {
            return None;
        }
        Some(self.destination_start + (value - self.source_start))
    }

    pub fn from_vec(vec: Vec<usize>) -> Self {
        Self {
            destination_start: vec[0],
            source_start: vec[1],
            length: vec[2],
        }
    }

    pub fn new(destination_start: usize, source_start: usize, length: usize) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self {
            destination_start: 0,
            source_start: 0,
            length: usize::MAX,
        }
    }
}

pub fn parse_input(input: &Vec<String>) -> Input {
    let seeds = input[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut range_store: Vec<Vec<Range>> = vec![Vec::new(); 7];
    let mut index = 0;

    for line in input.iter().skip(3) {
        if line.is_empty() {
            continue;
        }
        if line.chars().nth(0).unwrap().is_digit(10) {
            let range = Range::from_vec(
                line.split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
            range_store[index].push(range);
        } else {
            range_store[index].push(Range::default());
            index += 1;
        }
    }
    range_store[index].push(Range::default());
    Input { seeds, range_store }
}

pub mod part_1 {
    use super::*;

    pub fn convert_number(input: usize, ranges: &Vec<Range>) -> Option<usize> {
        for range in ranges {
            if let Some(result) = range.map(input) {
                return Some(result);
            }
        }
        None
    }

    pub fn find_closest_seed(input: &Vec<String>) -> usize {
        let input = parse_input(input);

        input
            .seeds
            .iter()
            .map(|s| {
                input
                    .range_store
                    .iter()
                    .fold(*s, |acc, ranges| convert_number(acc, ranges).unwrap())
            })
            .fold(usize::MAX, |acc, s| if s < acc { s } else { acc })
    }
}

pub mod part_2 {
    use super::*;

    pub fn convert_number(input: usize, ranges: &Vec<Range>) -> Option<usize> {
        for range in ranges {
            if let Some(result) = range.map(input) {
                return Some(result);
            }
        }
        None
    }

    // over counts by one and take forever to run :(
    pub fn find_closest_seed(input: &Vec<String>) -> usize {
        let input = parse_input(input);

        // let lowest_location_range = input
        //     .range_store
        //     .iter()
        //     .rev()
        //     .nth(0)
        //     .unwrap()
        //     .iter()
        //     .reduce(|acc, r| {
        //         if r.destination_start < acc.source_start {
        //             r
        //         } else {
        //             acc
        //         }
        //     })
        //     .unwrap();
        //
        // let lowest_seed_range = input.range_store.iter().rev().skip(1).reduce(|acc, r| {
        //     r.
        // })

        (0..(input.seeds.len() / 2)).fold(usize::MAX, |acc, i| {
            let seed_range_start = input.seeds[i * 2];
            let seed_range_len = input.seeds[i * 2 + 1];

            let next = (seed_range_start..(seed_range_start + seed_range_len)).fold(
                usize::MAX,
                |acc, s| {
                    let next = input
                        .range_store
                        .iter()
                        .fold(s, |acc, ranges| convert_number(acc, ranges).unwrap());
                    if next < acc {
                        next
                    } else {
                        acc
                    }
                },
            );
            if next < acc {
                next
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::part_1::*;

        #[test]
        fn sample() {
            let input = vec![
                "seeds: 79 14 55 13".to_string(),
                "".to_string(),
                "seed-to-soil map:".to_string(),
                "50 98 2".to_string(),
                "52 50 48".to_string(),
                "".to_string(),
                "soil-to-fertilizer map:".to_string(),
                "0 15 37".to_string(),
                "37 52 2".to_string(),
                "39 0 15".to_string(),
                "".to_string(),
                "fertilizer-to-water map:".to_string(),
                "49 53 8".to_string(),
                "0 11 42".to_string(),
                "42 0 7".to_string(),
                "57 7 4".to_string(),
                "".to_string(),
                "water-to-light map:".to_string(),
                "88 18 7".to_string(),
                "18 25 70".to_string(),
                "".to_string(),
                "light-to-temperature map:".to_string(),
                "45 77 23".to_string(),
                "81 45 19".to_string(),
                "68 64 13".to_string(),
                "".to_string(),
                "temperature-to-humidity map:".to_string(),
                "0 69 1".to_string(),
                "1 0 69".to_string(),
                "".to_string(),
                "humidity-to-location map:".to_string(),
                "60 56 37".to_string(),
                "56 93 4".to_string(),
            ];
            let result = find_closest_seed(&input);
            assert_eq!(result, 35);
        }
    }

    mod part_2 {
        use super::super::part_2::*;

        #[test]
        fn sample() {
            let input = vec![
                "seeds: 79 14 55 13".to_string(),
                "".to_string(),
                "seed-to-soil map:".to_string(),
                "50 98 2".to_string(),
                "52 50 48".to_string(),
                "".to_string(),
                "soil-to-fertilizer map:".to_string(),
                "0 15 37".to_string(),
                "37 52 2".to_string(),
                "39 0 15".to_string(),
                "".to_string(),
                "fertilizer-to-water map:".to_string(),
                "49 53 8".to_string(),
                "0 11 42".to_string(),
                "42 0 7".to_string(),
                "57 7 4".to_string(),
                "".to_string(),
                "water-to-light map:".to_string(),
                "88 18 7".to_string(),
                "18 25 70".to_string(),
                "".to_string(),
                "light-to-temperature map:".to_string(),
                "45 77 23".to_string(),
                "81 45 19".to_string(),
                "68 64 13".to_string(),
                "".to_string(),
                "temperature-to-humidity map:".to_string(),
                "0 69 1".to_string(),
                "1 0 69".to_string(),
                "".to_string(),
                "humidity-to-location map:".to_string(),
                "60 56 37".to_string(),
                "56 93 4".to_string(),
            ];
            let result = find_closest_seed(&input);
            assert_eq!(result, 46);
        }
    }
}
