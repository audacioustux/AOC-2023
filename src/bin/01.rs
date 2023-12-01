use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).unwrap_or_default();
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .unwrap_or_default();

            let mut digits = String::with_capacity(2);
            digits.push(first);
            digits.push(last);

            digits.parse::<u32>().unwrap_or(0)
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits_map = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let canonicalized: &str = &input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            for (word, digit) in &digits_map {
                let replace = format!("{}{}{}", word, digit, word);
                line = line.replace(word, &replace);
            }
            line
        })
        .join("\n");

    part_one(&canonicalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
