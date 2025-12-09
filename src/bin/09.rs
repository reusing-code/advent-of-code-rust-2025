use std::{collections::HashMap, vec};

use advent_of_code::{Coord2D, DIRECTIONS, Field};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut red_tiles: Vec<Coord2D> = vec![];
    for (x, y) in input.lines().map(|x| {
        x.split_once(",")
            .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            .unwrap()
    }) {
        red_tiles.push(Coord2D { x, y });
    }
    let mut max: u64 = 0;
    for (i, t1) in red_tiles.iter().enumerate() {
        for t2 in red_tiles.iter().skip(i + 1) {
            let val = (((t1.x - t2.x).abs() + 1) * ((t1.y - t2.y).abs() + 1)) as u64;
            if val > max {
                max = val;
            }
        }
    }
    Some(max)
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut red_tiles: Vec<Coord2D> = vec![];
    let mut x_vals: Vec<i64> = vec![];
    let mut y_vals: Vec<i64> = vec![];
    for (x, y) in input.lines().map(|x| {
        x.split_once(",")
            .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            .unwrap()
    }) {
        red_tiles.push(Coord2D { x, y });
        x_vals.push(x);
        y_vals.push(y);
    }
    x_vals.sort();
    y_vals.sort();
    let mut x_mapping = HashMap::new();
    for (i, x) in x_vals.iter().enumerate() {
        x_mapping.insert(*x, i as i64);
    }
    let mut y_mapping = HashMap::new();
    for (i, y) in y_vals.iter().enumerate() {
        y_mapping.insert(*y, i as i64);
    }

    let mut red_tiles_mapped: Vec<Coord2D> = vec![];
    for r in red_tiles {
        red_tiles_mapped.push(Coord2D {
            x: *x_mapping.get(&r.x).unwrap(),
            y: *y_mapping.get(&r.y).unwrap(),
        });
    }
    part_two_internal(
        &mut red_tiles_mapped,
        (x_vals.len() as i64, y_vals.len() as i64),
        &x_vals,
        &y_vals,
    )
}

pub fn part_two_internal(
    red_tiles: &mut Vec<Coord2D>,
    max: (i64, i64),
    x_vals: &[i64],
    y_vals: &[i64],
) -> Option<u64> {
    let mut field = Field::<i32> {
        w: max.0 as usize,
        h: max.1 as usize,
        data: Vec::new(),
    };
    field.data.resize((max.0 * max.1) as usize, 0);
    red_tiles.push(red_tiles.first().unwrap().clone());
    for (i, t1) in red_tiles.iter().enumerate() {
        if let Some(t2) = red_tiles.get(i + 1) {
            if t1.x == t2.x {
                for y in t1.y.min(t2.y)..t1.y.max(t2.y) + 1 {
                    field.set_coord(&Coord2D { x: t1.x, y }, &1);
                }
            } else {
                for x in t1.x.min(t2.x)..t1.x.max(t2.x) + 1 {
                    field.set_coord(&Coord2D { x, y: t1.y }, &1);
                }
            }
        }
    }
    red_tiles.remove(red_tiles.len() - 1);
    let mut next: Vec<(i64, i64)> = vec![];
    for x in [0, field.w - 1] {
        for y in [0, field.h - 1] {
            if field.get(x, y).unwrap() == 0 {
                next.push((x as i64, y as i64));
            }
        }
    }

    while let Some(n) = next.pop() {
        let c = &Coord2D { x: n.0, y: n.1 };
        if let Some(m) = field.get_coord(c)
            && m == 0
        {
            field.set_coord(c, &2);
            for dir in DIRECTIONS {
                let new = &c.add(dir);
                next.push((new.x, new.y));
            }
        }
    }
    let mut max: u64 = 0;
    for (i, t1) in red_tiles.iter().enumerate() {
        'outer: for t2 in red_tiles.iter().skip(i + 1) {
            for x in t1.x.min(t2.x)..t1.x.max(t2.x) + 1 {
                for y in t1.y.min(t2.y)..t1.y.max(t2.y) + 1 {
                    if field.get_signed(x, y).unwrap() == 2 {
                        continue 'outer;
                    }
                }
            }
            let t11 = Coord2D {
                x: x_vals[t1.x as usize],
                y: y_vals[t1.y as usize],
            };
            let t22 = Coord2D {
                x: x_vals[t2.x as usize],
                y: y_vals[t2.y as usize],
            };
            let val = (((t11.x - t22.x).abs() + 1) * ((t11.y - t22.y).abs() + 1)) as u64;

            if val > max {
                max = val;
            }
        }
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
