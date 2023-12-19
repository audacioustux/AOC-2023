use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Beam {
    pos: Coord,
    dir: Direction,
}

impl Beam {
    fn forward(mut self, rows: usize, cols: usize) -> Option<Self> {
        match self.dir {
            Direction::Up if self.pos.y > 0 => self.pos.y -= 1,
            Direction::Down if self.pos.y < rows - 1 => self.pos.y += 1,
            Direction::Left if self.pos.x > 0 => self.pos.x -= 1,
            Direction::Right if self.pos.x < cols - 1 => self.pos.x += 1,
            _ => return None,
        }
        Some(self)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Tile {
    Empty,
    SplitHoriz,
    SplitVert,
    MirrorForward,
    MirrorBack,
}

fn parse(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '\\' => Tile::MirrorBack,
                    '/' => Tile::MirrorForward,
                    '.' => Tile::Empty,
                    '-' => Tile::SplitHoriz,
                    '|' => Tile::SplitVert,
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect()
}

fn energized(start: Beam, grid: &[Vec<Tile>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut q = VecDeque::new();
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();
    q.push_back(start);

    while let Some(mut beam) = q.pop_front() {
        if seen.contains(&beam) {
            continue;
        }
        energized.insert(beam.pos);
        seen.insert(beam);

        let dirs = match (grid[beam.pos.y][beam.pos.x], beam.dir) {
            (Tile::Empty, _)
            | (Tile::SplitHoriz, Direction::Left)
            | (Tile::SplitHoriz, Direction::Right)
            | (Tile::SplitVert, Direction::Up)
            | (Tile::SplitVert, Direction::Down) => vec![beam.dir],
            (Tile::SplitHoriz, _) => {
                vec![Direction::Left, Direction::Right]
            }
            (Tile::SplitVert, _) => {
                vec![Direction::Up, Direction::Down]
            }
            (Tile::MirrorForward, Direction::Up) | (Tile::MirrorBack, Direction::Down) => {
                vec![Direction::Right]
            }
            (Tile::MirrorForward, Direction::Down) | (Tile::MirrorBack, Direction::Up) => {
                vec![Direction::Left]
            }
            (Tile::MirrorForward, Direction::Left) | (Tile::MirrorBack, Direction::Right) => {
                vec![Direction::Down]
            }
            (Tile::MirrorForward, Direction::Right) | (Tile::MirrorBack, Direction::Left) => {
                vec![Direction::Up]
            }
        };
        for dir in dirs {
            beam.dir = dir;
            if let Some(beam) = beam.forward(rows, cols) {
                q.push_back(beam);
            }
        }
    }
    energized.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = Beam {
        pos: Coord { x: 0, y: 0 },
        dir: Direction::Right,
    };
    Some(energized(start, &grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let from_left = (0..grid.len()).map(|row| Beam {
        dir: Direction::Right,
        pos: Coord { x: 0, y: row },
    });
    let from_right = (0..grid.len()).map(|row| Beam {
        dir: Direction::Left,
        pos: Coord {
            x: grid[0].len() - 1,
            y: row,
        },
    });
    let from_up = (0..grid[0].len()).map(|col| Beam {
        dir: Direction::Down,
        pos: Coord { x: col, y: 0 },
    });
    let from_down = (0..grid[0].len()).map(|col| Beam {
        dir: Direction::Up,
        pos: Coord {
            x: col,
            y: grid.len() - 1,
        },
    });

    from_left
        .chain(from_right)
        .chain(from_up)
        .chain(from_down)
        .map(|start| energized(start, &grid))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
