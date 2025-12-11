use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(10);

type Cache = HashMap<(Vec<bool>, Vec<Vec<usize>>, usize), bool>;
type Cache2 = HashMap<(Vec<usize>, Vec<Vec<usize>>), usize>;

fn check_buttons_cache(
    current: Vec<bool>,
    goal: &[bool],
    buttons: Vec<Vec<usize>>,
    depth: usize,
    cache: &mut Cache,
) -> usize {
    if cache.contains_key(&(current.clone(), buttons.clone(), depth)) {
        return usize::MAX;
    }
    let res = check_buttons(current.clone(), goal, buttons.clone(), depth, cache);
    cache.insert((current, buttons, depth), true);
    res
}
fn check_buttons(
    current: Vec<bool>,
    goal: &[bool],
    buttons: Vec<Vec<usize>>,
    depth: usize,
    cache: &mut Cache,
) -> usize {
    if buttons.is_empty() {
        return usize::MAX;
    }
    let mut next: Vec<(Vec<bool>, Vec<Vec<usize>>, usize)> = vec![];
    for (i, b) in buttons.iter().enumerate() {
        let mut new_buttons = buttons.clone();
        new_buttons.remove(i);
        let mut new_current = current.clone();
        for j in b {
            new_current[*j] = !new_current[*j];
        }
        let diff = new_current
            .iter()
            .zip(goal.iter())
            .map(|(x, y)| if x == y { 0 } else { 1 })
            .sum::<usize>();
        if diff == 0 {
            return depth + 1;
        }
        next.push((new_current, new_buttons, diff));
    }
    next.sort_by_key(|k| k.2);
    let mut min: usize = usize::MAX;
    for n in next {
        let v = check_buttons_cache(n.0, goal, n.1, depth + 1, cache);
        if v == depth + 2 {
            return depth + 2;
        }
        if v < min {
            min = v;
        }
    }
    min
}

fn diff(goal: &[usize], buttons: &[Vec<usize>], button_presses: &[usize]) -> Option<usize> {
    let mut current = vec![0; goal.len()];
    for (i, b) in buttons.iter().enumerate() {
        let cnt = button_presses[i];
        for v in b {
            current[*v] += cnt;
        }
    }
    let mut diff = 0;
    for (i, v) in current.iter().enumerate() {
        if *v > goal[i] {
            return None;
        }
        diff += goal[i] - *v;
    }
    Some(diff)
}

fn check_joltage(goal: &[usize], buttons: &[Vec<usize>]) -> usize {
    let mut button_ranges: Vec<(usize, usize)> = vec![(0, usize::MAX); buttons.len()];

    for (i, &g) in goal.iter().enumerate() {
        let hitting_buttons = buttons
            .iter()
            .enumerate()
            .filter(|(_, x)| x.contains(&i))
            .map(|(a, _)| a);
        let num_buttons = buttons.iter().filter(|x| x.contains(&i)).count();
        for b in hitting_buttons {
            button_ranges[b].1 = button_ranges[b].1.min(g);
            if num_buttons == 1 {
                button_ranges[b].0 = button_ranges[b].0.max(g);
            }
        }
    }
    println!("ranges: {:?}", button_ranges);

    let mut combinations: Vec<_> = button_ranges
        .iter()
        .map(|&(min, max)| min..=max)
        .multi_cartesian_product()
        .collect();
    println!("combinations: {}", combinations.len());
    return 0;
    combinations.sort_by_key(|x| x.iter().sum::<usize>());
    let res = combinations
        .iter()
        .find(|comb| diff(goal, buttons, comb) == Some(0));
    res.unwrap().iter().sum::<usize>()
}

fn parse(input: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<usize>) {
    let (light_start, light_end) = (input.find("[").unwrap() + 1, input.find("]").unwrap());
    let (joltage_tart, joltage_end) = (input.find("{").unwrap() + 1, input.find("}").unwrap());
    let mut light = vec![];
    for c in input[light_start..light_end].chars() {
        if c == '.' {
            light.push(false);
        } else if c == '#' {
            light.push(true);
        }
    }
    let joltage: Vec<_> = input[joltage_tart..joltage_end]
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    let mut buttons = vec![];

    let mut s = &input[light_end + 1..joltage_tart - 1];
    while !s.is_empty() {
        if let (Some(button_start), Some(button_end)) = (s.find("("), s.find(")")) {
            buttons.push(
                s[button_start + 1..button_end]
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            );
            s = &s[button_end + 1..];
        } else {
            break;
        }
    }
    (light, buttons, joltage)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let (lights, buttons, _) = parse(line);
        let mut current = vec![];
        current.resize(lights.len(), false);
        let mut cache = Cache::new();
        result += check_buttons_cache(current, &lights, buttons, 0, &mut cache);
    }
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        println!("###");
        let (_, mut buttons, jolt) = parse(line);
        buttons.sort_by_key(|x| x.len());
        buttons.reverse();
        result += check_joltage(&jolt, &buttons);
    }
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
