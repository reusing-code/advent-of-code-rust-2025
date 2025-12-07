use advent_of_code::Field;

advent_of_code::solution!(6);
pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let mut field: Vec<Vec<&str>> = vec![];
    input
        .lines()
        .map(|x| field.push(x.split_ascii_whitespace().collect::<Vec<&str>>()))
        .count();
    for (i, s) in field[field.len() - 1].iter().enumerate() {
        if *s == "+" {
            result += field
                .iter()
                .rev()
                .skip(1)
                .map(|x| x[i].parse::<u64>().unwrap())
                .sum::<u64>();
        } else {
            result += field
                .iter()
                .rev()
                .skip(1)
                .map(|x| x[i].parse::<u64>().unwrap())
                .product::<u64>();
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let field = Field::<char>::new(input);
    let columns: Vec<_> = input
        .lines()
        .next_back()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .collect();
    for (i, (j, c)) in columns.iter().enumerate() {
        let mut next = 0;
        if i + 1 == columns.len() {
            next = input.lines().next().unwrap().chars().count() + 1;
        } else {
            next = columns[i + 1].0;
        }
        let mut numbers: Vec<u64> = vec![];
        for k in *j..next - 1 {
            let mut num = 0;
            for l in 0..field.h - 1 {
                let ch = field.get(k, l).unwrap().to_digit(10);
                ch.and_then(|x| -> Option<u32> {
                    num = num * 10 + x as u64;
                    None
                });
            }
            numbers.push(num);
        }
        if *c == '+' {
            result += numbers.iter().sum::<u64>();
        } else {
            result += numbers.iter().product::<u64>();
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
