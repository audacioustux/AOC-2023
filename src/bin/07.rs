use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .sorted_by(|(a, _), (b, _)| {
            let card_value = |c: char| "23456789TJQKA".chars().position(|card| card == c).unwrap();
            let hand_score = |hand: &str| {
                let mut faces = [0; 13];
                for c in hand.chars() {
                    faces[card_value(c)] += 1;
                }
                faces.sort_unstable();
                faces.reverse();
                faces
            };

            hand_score(a).cmp(&hand_score(b)).then_with(|| {
                let (a, b) = a.chars().zip(b.chars()).find(|(a, b)| a != b).unwrap();

                card_value(a).cmp(&card_value(b))
            })
        })
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid.parse::<usize>().unwrap())
        .sum1()
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .sorted_by(|(a, _), (b, _)| {
            let card_value = |c: char| "J23456789TQKA".chars().position(|card| card == c).unwrap();
            let hand_score = |hand: &str| {
                let (mut faces, jokers) =
                    hand.chars().fold(([0; 13], 0), |(mut faces, jokers), c| {
                        if c == 'J' {
                            (faces, jokers + 1)
                        } else {
                            faces[card_value(c)] += 1;
                            (faces, jokers)
                        }
                    });

                faces.sort_unstable();
                faces.reverse();
                faces[0] += jokers;
                faces
            };
            hand_score(a).cmp(&hand_score(b)).then_with(|| {
                let (a, b) = a.chars().zip(b.chars()).find(|(a, b)| a != b).unwrap();

                card_value(a).cmp(&card_value(b))
            })
        })
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid.parse::<usize>().unwrap())
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
