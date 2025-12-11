use std::collections::HashMap;

advent_of_code::solution!(11);

fn path_rec(curr: &str, map: &HashMap<&str, Vec<&str>>, cache: &mut HashMap<String, u64>) -> u64 {
    if let Some(v) = cache.get(curr) {
        return *v;
    }
    if curr == "out" {
        return 1;
    }
    let mut result = 0;
    for &next in map.get(curr).unwrap() {
        result += path_rec(next, map, cache);
    }
    cache.insert(curr.to_string(), result);
    result
}

fn path_rec_two(
    curr: &str,
    map: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<String, (u64, u64, u64, u64)>,
) -> (u64, u64, u64, u64) {
    if let Some(v) = cache.get(curr) {
        return *v;
    }
    if curr == "out" {
        return (0, 0, 0, 1);
    }
    let mut result = (0, 0, 0, 0);
    let fft = curr == "fft";
    let dac = curr == "dac";
    for &next in map.get(curr).unwrap() {
        let (b, f, d, n) = path_rec_two(next, map, cache);
        if !fft && !dac {
            result.0 += b;
            result.1 += f;
            result.2 += d;
            result.3 += n;
        }
        if fft {
            result.0 += d;
            result.1 += n;
            result.2 = 0;
            result.3 = 0;
        }
        if dac {
            result.0 += f;
            result.1 = 0;
            result.2 += n;
            result.3 = 0;
        }
    }

    cache.insert(curr.to_string(), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = HashMap::<&str, Vec<&str>>::new();
    for line in input.lines() {
        let (from_str, to_str) = line.split_once(":").unwrap();
        map.insert(
            from_str.trim(),
            to_str.split_whitespace().collect::<Vec<&str>>(),
        );
    }
    let mut cache = HashMap::new();
    Some(path_rec("you", &map, &mut cache))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = HashMap::<&str, Vec<&str>>::new();
    for line in input.lines() {
        let (from_str, to_str) = line.split_once(":").unwrap();
        map.insert(
            from_str.trim(),
            to_str.split_whitespace().collect::<Vec<&str>>(),
        );
    }
    let mut cache = HashMap::new();
    let result = path_rec_two("svr", &map, &mut cache);
    Some(result.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
