use itertools::Itertools;
advent_of_code::solution!(15);

enum Instruction<'a> {
    Remove(&'a str),
    Add(Lens<'a>),
}

impl<'a> Instruction<'a> {
    fn new(s: &'a str) -> Self {
        if let Some(label) = s.strip_suffix('-') {
            Self::Remove(label)
        } else {
            let (label, focal) = s.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let lens = Lens { label, focal };
            Self::Add(lens)
        }
    }
}
struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, byte| acc.wrapping_add(byte).wrapping_mul(17))
}

pub fn part_one(input: &str) -> Option<u32> {
    input.trim().split(',').map(|s| hash(s) as u32).sum1()
}

pub fn part_two(input: &str) -> Option<usize> {
    const BOX: Vec<Lens> = Vec::new();
    let mut boxes = [BOX; 256];

    for instr in input.trim_end().split(',').map(Instruction::new) {
        match instr {
            Instruction::Remove(label) => {
                let hash = hash(label);
                boxes[hash as usize].retain(|item| item.label != label);
            }
            Instruction::Add(lens) => {
                let hash = hash(lens.label);
                let lenses = &mut boxes[hash as usize];
                if let Some(old) = lenses.iter_mut().find(|item| lens.label == item.label) {
                    old.focal = lens.focal;
                } else {
                    lenses.push(lens);
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            let box_focusing_power: usize = lenses
                .into_iter()
                .enumerate()
                .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal as usize)
                .sum();
            box_focusing_power
        })
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
