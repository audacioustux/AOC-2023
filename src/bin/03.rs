use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    #[derive(Debug, Clone, Copy)]
    enum Cord {
        Symbol(char, (usize, usize)),
        Number(u32, (usize, usize)),
    }
    let indexes: Vec<Cord> = input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            let symbols = line
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_numeric() && c != &'.')
                .map(move |(char_index, symbol)| Cord::Symbol(symbol, (line_index, char_index)));

            let digits_grouped = line.chars().enumerate().group_by(|(_, c)| c.is_numeric());
            let numbers = digits_grouped
                .into_iter()
                .filter(|(is_digit, _)| *is_digit)
                .map(move |(_, group)| {
                    let mut group = group.peekable();
                    let char_index = group.peek().unwrap().0;
                    let number = group
                        .map(|(_, c)| c)
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    Cord::Number(number, (line_index, char_index))
                });

            symbols.chain(numbers).collect::<Vec<_>>()
        })
        .collect();

    let neighbors = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(x, y)| *x != 0 || *y != 0)
        .collect::<Vec<_>>();

    // store the numbers with their span (first and last index) in a way to efficiently find if a index is in a span or not

    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
