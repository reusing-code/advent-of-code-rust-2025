use advent_of_code::split_by_empt_line;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let split = split_by_empt_line(input);
    let mut ranges: Vec<(u64, u64)> = vec![];
    split[0]
        .iter()
        .map(|x| {
            let s = x
                .split("-")
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            ranges.push((s[0], s[1]));
        })
        .count();

    Some(
        split[1]
            .iter()
            .filter(|&x| {
                let val = x.parse::<u64>().unwrap();
                ranges.iter().any(|(a, b)| val >= *a && val <= *b)
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let split = split_by_empt_line(input);
    let mut ranges: Vec<(u64, u64)> = vec![];
    split[0]
        .iter()
        .map(|x| {
            let s = x
                .split("-")
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            ranges.push((s[0], s[1]));
        })
        .count();

    ranges.sort_by(|(a1, _), (b1, _)| a1.cmp(b1));

    let mut final_ranges: Vec<(u64, u64)> = vec![];
    let mut i = 1;
    let mut last = ranges[0];
    while i < ranges.len() {
        if last.1 < ranges[i].0 {
            final_ranges.push(last);
            last = ranges[i];
        } else {
            last.1 = last.1.max(ranges[i].1)
        }
        i += 1;
    }
    final_ranges.push(last);

    Some(final_ranges.iter().map(|(a, b)| *b - *a + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
