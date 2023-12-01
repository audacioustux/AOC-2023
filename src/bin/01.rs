use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_ascii_digit());
            let first = digits.clone().next().unwrap_or_default();
            let last = digits.clone().last().unwrap_or_default();

            format!("{}{}", first, last).parse().unwrap_or(0)
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let digits_word = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let canonicalized: &str = &input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            for (word, digit) in digits_word.iter().zip(1..=9) {
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
