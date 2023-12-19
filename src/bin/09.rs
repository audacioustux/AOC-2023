use itertools::Itertools;

advent_of_code::solution!(9);

fn nums(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn differences(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .tuple_windows()
        .map(|(left, right)| right - left)
        .collect()
}

fn next_num(nums: &[i32]) -> i32 {
    if nums.iter().all(|&n| n == 0) {
        return 0;
    }
    let differences: Vec<i32> = differences(nums);
    next_num(&differences) + nums.iter().last().unwrap()
}

pub fn part_one(input: &str) -> Option<i32> {
    input.lines().map(|line| next_num(&nums(line))).sum1()
}

fn prev_num(nums: &[i32]) -> i32 {
    if nums.iter().all(|&n| n == 0) {
        return 0;
    }
    let differences = differences(nums);
    nums[0] - prev_num(&differences)
}

pub fn part_two(input: &str) -> Option<i32> {
    input.lines().map(|line| prev_num(&nums(line))).sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
