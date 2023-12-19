advent_of_code::solution!(18);

struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn advance(&self, direction: &Dir, amount: i64) -> Self {
        match direction {
            Dir::Up => Self {
                x: self.x + amount,
                y: self.y,
            },
            Dir::Down => Self {
                x: self.x - amount,
                y: self.y,
            },
            Dir::Left => Self {
                x: self.x,
                y: self.y - amount,
            },
            Dir::Right => Self {
                x: self.x,
                y: self.y + amount,
            },
        }
    }
}

struct Instr {
    dir: Dir,
    amount: i64,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn calc_area(instructions: impl Iterator<Item = Instr>) -> i64 {
    let (area, perimeter, _) = instructions.fold(
        (0, 0, Coord { x: 0, y: 0 }),
        |(area, perimeter, pos), Instr { dir, amount }| {
            let new_pos = pos.advance(&dir, amount);
            let new_area = area + (pos.x * new_pos.y - new_pos.x * pos.y);
            let new_perimeter = (new_pos.x - pos.x).abs() + (new_pos.y - pos.y).abs() + perimeter;
            (new_area, new_perimeter, new_pos)
        },
    );

    (area.abs() + perimeter) / 2 + 1
}

pub fn part_one(input: &str) -> Option<i64> {
    let instructions = input.lines().map(|line| {
        let (instr, _) = line.split_once(" (").unwrap();
        let (dir, amount) = instr.split_once(" ").unwrap();
        let dir = match dir {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("wrong dir"),
        };
        let amount = amount.parse().unwrap();
        Instr { dir, amount }
    });

    Some(calc_area(instructions))
}

pub fn part_two(input: &str) -> Option<i64> {
    let instructions = input.lines().map(|line| {
        let line = line.strip_suffix(")").unwrap();
        let (_, hex) = line.split_once("(#").unwrap();
        let (amount, dir) = hex.split_at(5);
        let amount = i64::from_str_radix(amount, 16).unwrap();
        let dir = match dir {
            "3" => Dir::Up,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "0" => Dir::Right,
            _ => panic!("wrong dir"),
        };
        Instr { dir, amount }
    });

    Some(calc_area(instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
