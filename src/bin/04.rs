use advent_of_code::{Coord2D, DIRECTIONS_DIAG, Field};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let field = Field::<char>::new(input);
    let mut result = 0;
    for y in 0..field.h {
        for x in 0..field.w {
            if field.get(x, y).unwrap() == '@' {
                let current = Coord2D {
                    x: x as i64,
                    y: y as i64,
                };
                let count = DIRECTIONS_DIAG
                    .iter()
                    .filter(|co| {
                        field
                            .get_coord(&co.add(&current))
                            .map(|c| c == '@')
                            .unwrap_or(false)
                    })
                    .count();
                if count < 4 {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut field = Field::<char>::new(input);
    let mut result = 0;
    let mut y = 0;
    while y < field.h {
        let mut x = 0;
        while x < field.w {
            if field.get(x, y).unwrap() == '@' {
                let current = Coord2D {
                    x: x as i64,
                    y: y as i64,
                };
                let count = DIRECTIONS_DIAG
                    .iter()
                    .filter(|co| {
                        field
                            .get_coord(&co.add(&current))
                            .map(|c| c == '@')
                            .unwrap_or(false)
                    })
                    .count();
                if count < 4 {
                    result += 1;
                    field.set_coord(&current, &'.');
                    x = x.saturating_sub(1);
                    y = y.saturating_sub(1);
                } else {
                    x += 1;
                }
            } else {
                x += 1;
            }
        }
        y += 1;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
