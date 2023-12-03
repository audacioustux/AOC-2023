use std::{
    collections::{BTreeSet, HashMap},
    ops::RangeInclusive,
};

use itertools::Itertools;

advent_of_code::solution!(3);

fn parse(
    input: &str,
) -> impl Iterator<Item = (BTreeSet<usize>, Vec<(usize, RangeInclusive<usize>)>)> + '_ {
    input.lines().map(|line| {
        let symbols = line
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_numeric() && c != &'.')
            .map(|(char_index, _)| char_index);

        let digits_grouped = line.chars().enumerate().group_by(|(_, c)| c.is_numeric());
        let numbers = digits_grouped
            .into_iter()
            .filter(|(is_digit, _)| *is_digit)
            .map(move |(_, group)| {
                let mut group = group.peekable();
                let first_digit_index = group.peek().unwrap().0;
                let last_digit_index = group.last().unwrap().0;
                let number = line[first_digit_index..=last_digit_index]
                    .parse::<usize>()
                    .unwrap();
                (number, first_digit_index..=last_digit_index)
            });

        (symbols.collect::<BTreeSet<_>>(), Vec::from_iter(numbers))
    })
}

fn gear_parts_map<P>(l1: P, l2: P, l3: P, symbols: &BTreeSet<usize>) -> HashMap<&usize, Vec<usize>>
where
    P: IntoIterator<Item = (usize, RangeInclusive<usize>)>,
{
    let parts = l1
        .into_iter()
        .chain(l2.into_iter())
        .chain(l3.into_iter())
        .fold(HashMap::new(), |mut acc, (number, range)| {
            let range = range.start().saturating_sub(1)..=range.end().saturating_add(1);
            let symbols = symbols.range(range);
            for symbol in symbols {
                acc.entry(symbol).or_insert_with(Vec::new).push(number);
            }
            acc
        });

    parts
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input)
        .tuple_windows::<(_, _, _)>()
        .map(|((_, first_line), (symbols, curr_line), (_, last_line))| {
            let gear_parts = gear_parts_map(first_line, curr_line, last_line, &symbols);
            gear_parts.values().flatten().sum::<usize>() as u32
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input)
        .tuple_windows::<(_, _, _)>()
        .flat_map(|((_, first_line), (symbols, curr_line), (_, last_line))| {
            let gear_parts = gear_parts_map(first_line, curr_line, last_line, &symbols);
            gear_parts
                .iter()
                .filter(|(_, v)| v.len() == 2)
                .map(|(_, v)| (v[0] * v[1]) as u32)
                .collect_vec()
        })
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
