use std::collections::HashMap;

use advent_of_code::{Coord2D, Field};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut field = Field::<char>::new(input);
    let mut i = 0;
    while i < field.data.len() {
        let c = field.data[i];
        let current = field.coord_from_index(i as i64);
        let top_beam = field.get_coord(&current.add(&Coord2D { x: 0, y: -1 })) == Some('|');
        match c {
            'S' => field.set_coord(&current, &'|'),
            '.' => {
                if top_beam {
                    field.set_coord(&current, &'|');
                }
            }
            '^' => {
                if top_beam {
                    result += 1;
                    if let Some(v) = field.get_coord_mut(&current.add(&Coord2D { x: -1, y: 0 })) {
                        *v = '|';
                    }
                    if let Some(v) = field.get_coord_mut(&current.add(&Coord2D { x: 1, y: 0 })) {
                        *v = '|';
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current_beams: HashMap<usize, u64> = HashMap::new();
    let mut last_beams: HashMap<usize, u64>;
    for line in input.lines() {
        last_beams = current_beams;
        current_beams = HashMap::new();
        for (i, c) in line.chars().enumerate() {
            let top_val = last_beams.get(&i);
            match c {
                'S' => {
                    current_beams.insert(i, 1);
                }
                '.' => {
                    if let Some(top_val1) = top_val {
                        current_beams
                            .entry(i)
                            .and_modify(|e| *e += *top_val1)
                            .or_insert(*top_val1);
                    }
                }
                '^' => {
                    if let Some(top_val1) = top_val {
                        if i > 0 {
                            current_beams
                                .entry(i - 1)
                                .and_modify(|e| *e += *top_val1)
                                .or_insert(*top_val1);
                        }
                        if i < line.len() - 1 {
                            current_beams
                                .entry(i + 1)
                                .and_modify(|e| *e += *top_val1)
                                .or_insert(*top_val1);
                        }
                    }
                }
                _ => (),
            }
        }
    }
    Some(current_beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
