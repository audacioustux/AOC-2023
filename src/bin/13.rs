use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

fn parse(input: &str) -> Vec<VecDeque<Vec<Tile>>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Tile::Ash,
                            '#' => Tile::Rock,
                            _ => panic!("at the disco"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn reflects_at(grid: &VecDeque<Vec<Tile>>, smudges: usize) -> Option<usize> {
    (1..grid.len()).find(|&offset| {
        let half1 = grid.iter().take(offset).rev();
        let half2 = grid.iter().skip(offset);
        let combined = half1.zip(half2); // the shortest half determines how long this is!
        let found_smudges: usize = combined
            .map(|(row1, row2)| row1.iter().zip(row2.iter()).filter(|(a, b)| a != b).count())
            .sum();

        found_smudges == smudges
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    grid.iter()
        .map(|grid| {
            if let Some(i) = reflects_at(grid, 0) {
                return i * 100;
            }
            let cols = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect())
                .collect();
            if let Some(i) = reflects_at(&cols, 0) {
                return i;
            }

            0
        })
        .sum1()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    grid.iter()
        .map(|grid| {
            if let Some(i) = reflects_at(grid, 1) {
                return i * 100;
            }

            let cols = (0..grid[0].len())
                .map(|i| grid.iter().map(|row| row[i]).collect())
                .collect();
            if let Some(i) = reflects_at(&cols, 1) {
                return i;
            }

            0
        })
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
