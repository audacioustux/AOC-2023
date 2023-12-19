advent_of_code::solution!(8);

use std::collections::HashMap;

enum Instruction {
    Left,
    Right,
}

struct Destinations<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().map(|c| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("at the disco"),
    });
    let map: HashMap<&str, Destinations> = map
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(" = ").unwrap();
            let destinations = destinations
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let destinations = destinations.split_once(", ").unwrap();
            (
                source,
                Destinations {
                    left: destinations.0,
                    right: destinations.1,
                },
            )
        })
        .collect();

    let mut instructions = instructions.cycle();
    let mut steps = 0;
    let mut curr = "AAA";

    while curr != "ZZZ" {
        let ins = instructions.next().unwrap();
        let Destinations { left, right } = map.get(curr).unwrap();
        curr = match ins {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        steps += 1;
    }

    Some(steps)
}

struct Ghost<'a> {
    pos: &'a str,
    cycles: Option<u64>,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = instructions
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect();
    let map: HashMap<&str, Destinations> = map
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(" = ").unwrap();
            let destinations = destinations
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let destinations = destinations.split_once(", ").unwrap();
            (
                source,
                Destinations {
                    left: destinations.0,
                    right: destinations.1,
                },
            )
        })
        .collect();

    let mut cycle_count = 0;
    let mut ghosts: Vec<Ghost> = map
        .keys()
        .filter(|source| source.ends_with('A'))
        .map(|pos| Ghost { pos, cycles: None })
        .collect();

    while ghosts.iter().any(|ghost| ghost.cycles.is_none()) {
        for ins in &instructions {
            for Ghost { pos, cycles } in ghosts.iter_mut() {
                if cycles.is_some() {
                    continue;
                }
                let Destinations { left, right } = map.get(pos).unwrap();
                *pos = match ins {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }
        }
        cycle_count += 1;

        for Ghost { pos, cycles: cycle } in ghosts.iter_mut() {
            if cycle.is_some() {
                continue;
            }
            if pos.ends_with('Z') {
                *cycle = Some(cycle_count);
            }
        }
    }

    let min_shared_cycles = ghosts
        .into_iter()
        .filter_map(|ghost| ghost.cycles)
        .fold(1, lcm);

    Some(min_shared_cycles * instructions.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
