use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::separated_list1, sequence::tuple, IResult,
};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let res: u32 = input
        .lines()
        .filter_map(|line| {
            let (_, (_, game_id, _, sets)) = nom_game_record(line).unwrap();
            sets.iter()
                .all(|set| {
                    set.iter()
                        .all(|(count, _, color)| count <= bag.get(color).unwrap())
                })
                .then_some(game_id)
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: u32 = input
        .lines()
        .map(|line| {
            let (_, (_, _, _, sets)) = nom_game_record(line).unwrap();
            sets.iter()
                .fold(HashMap::new(), |mut acc, set| {
                    set.iter().for_each(|(count, _, color)| {
                        let entry = acc.entry(color).or_insert(0);
                        *entry = (*entry).max(*count);
                    });
                    acc
                })
                .values()
                .product::<u32>()
        })
        .sum();

    Some(res)
}

fn nom_game_record(input: &str) -> IResult<&str, (&str, u32, &str, Vec<Vec<(u32, &str, &str)>>)> {
    tuple((
        tag("Game "),
        map_res(digit1, u32::from_str),
        tag(": "),
        separated_list1(
            tag("; "),
            separated_list1(
                tag(", "),
                tuple((
                    map_res(digit1, u32::from_str),
                    tag(" "),
                    alt((tag("blue"), tag("red"), tag("green"))),
                )),
            ),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
