advent_of_code::solution!(3);

pub fn joltage_bank(input: &str) -> u64 {
    let ve = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut max = 0;
    let mut max_idx = 0;
    for (i, v) in ve.iter().enumerate() {
        if i == ve.len() - 1 {
            continue;
        }
        if *v > max {
            max = *v;
            max_idx = i;
        }
    }
    let mut max2 = 0;
    for v in ve.iter().skip(max_idx + 1) {
        if *v > max2 {
            max2 = *v;
        }
    }

    (max * 10 + max2) as u64
}

pub fn joltage_rec(ve: &[u32], c: usize, val: u64) -> u64 {
    if c == 0 {
        return val;
    }
    let mut max: u64 = 0;
    let mut max_idx = 0;
    for (i, v) in ve.iter().enumerate() {
        if i > ve.len() - c {
            break;
        }
        if *v as u64 > max {
            max = *v as u64;
            max_idx = i;
        }
    }
    joltage_rec(&ve[max_idx + 1..], c - 1, val * 10 + max)
}

pub fn joltage_bank_two(input: &str) -> u64 {
    let ve = input
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    joltage_rec(&ve, 12, 0)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(joltage_bank).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(joltage_bank_two).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
