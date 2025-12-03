advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut current = 50;
    input
        .lines()
        .map(|x| {
            let positive = x.chars().take(1).collect::<String>() == "R";
            let mut val = x
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if !positive {
                val = -val;
            }
            current += val;
            current %= 100;
            if current < 0 {
                current += 100
            }

            if current == 0 {
                result += 1;
            }
        })
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut current = 50;
    input
        .lines()
        .map(|x| {
            let positive = x.chars().take(1).collect::<String>() == "R";
            let mut val = x
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            result += val / 100;
            if !positive {
                val = -val;
            }
            let before = current;
            current += val;
            current %= 100;
            if current < 0 {
                current += 100
            }
            if current < before && positive && current != 0 && before != 0 {
                result += 1;
            }
            if current > before && !positive && current != 0 && before != 0 {
                result += 1;
            }
            if current == 0 && val != 0 {
                result += 1;
            }
        })
        .count();
    Some(result as u64)
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
        assert_eq!(result, Some(6));
    }
}
