use itertools::Itertools;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut input = input.lines();
    let seeds = {
        let input = input.next().unwrap();
        input
            .split_once("seeds: ")
            .unwrap()
            .1
            .split(" ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec()
    };

    let maps = input
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(delim, cats)| (!delim).then_some(cats))
        .map(|cat| {
            cat.skip(1)
                .map(|maps| {
                    maps.splitn(3, " ")
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect_tuple::<(_, _, _)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    (seeds, maps)
}

fn lowest_location(
    seeds: impl Iterator<Item = u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
) -> Option<u32> {
    seeds
        .map(|seed| {
            maps.iter().fold(seed, |source_target, map| {
                map.iter()
                    .find_map(|(destination, source, range)| {
                        (destination + source_target).checked_sub(*source).and_then(
                            |mapped_destination| {
                                (*destination..=(destination + range))
                                    .contains(&mapped_destination)
                                    .then_some(mapped_destination)
                            },
                        )
                    })
                    .unwrap_or(source_target)
            }) as u32
        })
        .min()
}
pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse(input);
    lowest_location(seeds.into_iter(), maps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps) = parse(input);
    let seeds = seeds
        .iter()
        .tuples()
        .inspect(|(start, end)| println!("seed: {}..={}", start, end))
        .map(|(start, count)| (*start..(*start + *count)).collect_vec())
        .flatten();

    lowest_location(seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
