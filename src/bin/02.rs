advent_of_code::solution!(2);

fn is_invalid_one(val: u64) -> bool {
    let s = val.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }
    s.chars()
        .zip(s.chars().skip(s.len() / 2))
        .all(|(x, y)| x == y)
}

fn is_invalid_parts(val: u64, p: usize, len: usize) -> bool {
    let l = (len / p) as u32;
    let mut curr = val;
    let comp = val % 10u64.pow(l);
    for _ in 0..p {
        if comp != (curr % 10u64.pow(l)) {
            return false;
        }
        curr /= 10u64.pow(l);
    }
    true
}

fn is_invalid_two(val: u64) -> bool {
    let s = val.to_string();
    for p in 2..s.len() + 1 {
        if s.len().is_multiple_of(p) && is_invalid_parts(val, p, s.len()) {
            return true;
        }
    }
    false
}

fn next_invalid_one(val: u64) -> u64 {
    let s = val.to_string();
    let hlpow = 10u64.pow(s.len() as u32 / 2);
    if !s.len().is_multiple_of(2) {
        return 10u64.pow(s.len() as u32) + hlpow;
    }
    let front = val / hlpow;
    let back = val % hlpow;
    if back < front {
        return front * hlpow + front;
    }
    if (front + 1).to_string().len() > s.len() / 2 {
        return (front + 1) * hlpow * 10 + front + 1;
    }
    (front + 1) * hlpow + front + 1
}

fn add_invalid_ids_one(input: &str) -> u64 {
    let mut result = 0;
    let vec = input
        .split("-")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut current = vec[0];
    if is_invalid_one(current) {
        result += current;
    }
    current = next_invalid_one(current);
    while current <= vec[1] {
        result += current;
        current = next_invalid_one(current)
    }

    result
}
fn add_invalid_ids_two(input: &str) -> u64 {
    let vec = input
        .split("-")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    (vec[0]..vec[1] + 1).filter(|x| is_invalid_two(*x)).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(add_invalid_ids_one)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(add_invalid_ids_two)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_next_invalid() {
        assert_eq!(next_invalid_one(999), 1010);
        assert_eq!(next_invalid_one(9999), 100100);
        assert_eq!(next_invalid_one(150), 1010);
        assert_eq!(next_invalid_one(2000), 2020);
        assert_eq!(next_invalid_one(96), 99);
        assert_eq!(next_invalid_one(20), 22);
    }
}
