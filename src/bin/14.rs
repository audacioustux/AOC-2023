advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Round,
    Square,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Square,
                    'O' => Tile::Round,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn slide_north(grid: &mut Vec<Vec<Tile>>) {
    for col in 0..grid[0].len() {
        let mut empty_or_round_row = 0;
        for row in 0..grid.len() {
            let curr = grid[row][col];
            match curr {
                Tile::Square => empty_or_round_row = row + 1,
                Tile::Round => {
                    // swap the current tile with the empty_or_round one
                    let replace_with = std::mem::replace(&mut grid[empty_or_round_row][col], curr);
                    let _ = std::mem::replace(&mut grid[row][col], replace_with);
                    empty_or_round_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn weight(grid: &Vec<Vec<Tile>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let round_rocks = row.iter().filter(|tile| **tile == Tile::Round).count();
            round_rocks * (i + 1)
        })
        .sum()
}

// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn clockwise(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = grid.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn cycle(mut grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        slide_north(&mut grid);
        let rotated = clockwise(&grid);
        grid = rotated;
    }
    grid
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    slide_north(&mut grid);
    Some(weight(&grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let mut seen = vec![grid.clone()];

    let res = loop {
        grid = cycle(grid);
        // check if the cycled map has already been seen
        if let Some(idx) = seen.iter().position(|x| x == &grid) {
            // figure out length of cycle (watch out: a cycle might only start after a number of steps)
            let cycle_len = seen.len() - idx;
            // use cycle length to figure out the index of the final step in the seen list
            let final_idx = idx + (1_000_000_000 - idx) % cycle_len;
            break weight(&seen[final_idx]);
        }
        seen.push(grid.clone());
    };

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
