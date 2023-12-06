use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, records) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    times
        .iter()
        .zip(records)
        .map(|(time, record)| {
            (1u64..*time)
                .map(|t| t * (*time - t))
                .filter(|t| *t > record)
                .count() as u32
        })
        .product1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, record) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect_vec()
                .join("")
                .parse()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let product = (1u64..time)
        .map(|t| t * (time - t))
        .filter(|t| *t > record)
        .count();

    Some(product as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result,Some(71503));
    }
}
