use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(4);

fn wins(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| {
        let (_, (_, _, _, _, _, (have, winning))) = nom_card(line).unwrap();
        let winning_numbers = HashSet::<&u32>::from_iter(winning.iter());
        have.iter().filter(|&n| winning_numbers.contains(n)).count() as u32
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    wins(input)
        .map(|won| if won > 1 { 1 << (won - 1) } else { won })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    let wins: Vec<u32> = wins(input).collect();
    let original_num_of_card = wins.len();
    wins.iter()
        .enumerate()
        .fold(
            vec![1; original_num_of_card],
            |mut total_num_of_card, (index, won)| {
                if *won > 0 {
                    ((index + 1)..=(index + *won as usize))
                        .for_each(|i| total_num_of_card[i] += total_num_of_card[index]);
                }
                total_num_of_card
            },
        )
        .iter()
        .sum1()
}

fn nom_card(input: &str) -> IResult<&str, (&str, &str, u32, &str, &str, (Vec<u32>, Vec<u32>))> {
    tuple((
        tag("Card "),
        multispace0,
        map_res(digit1, u32::from_str),
        tag(": "),
        multispace0,
        separated_pair(
            separated_list1(multispace1, map_res(digit1, u32::from_str)),
            tuple((tag(" | "), multispace0)),
            separated_list1(multispace1, map_res(digit1, u32::from_str)),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
