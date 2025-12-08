advent_of_code::solution!(8);

fn dist(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}
fn find_in_clusters(n: usize, clusters: &[Box<Vec<usize>>]) -> Option<usize> {
    for c in clusters.iter().enumerate() {
        if c.1.contains(&n) {
            return Some(c.0);
        }
    }
    None
}

fn part_one_internal(input: &str, steps: usize) -> Option<u64> {
    let boxes: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|x| {
            let spl = x
                .split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (spl[0], spl[1], spl[2])
        })
        .collect();
    let mut distances: Vec<(i64, usize, usize)> = vec![];
    for (i, b1) in boxes.iter().enumerate() {
        for (j, b2) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((dist(b1, b2), i, j));
        }
    }
    distances.sort_by_key(|a| a.0);

    let mut clusters: Vec<Box<Vec<usize>>> = vec![];
    for dist in distances.iter().take(steps) {
        let fc1 = find_in_clusters(dist.1, &clusters);
        let fc2 = find_in_clusters(dist.2, &clusters);
        match fc1 {
            None => match fc2 {
                None => {
                    clusters.push(Box::new(vec![dist.1, dist.2]));
                }
                Some(c2) => clusters[c2].push(dist.1),
            },
            Some(c1) => match fc2 {
                None => {
                    clusters[c1].push(dist.2);
                }
                Some(c2) => {
                    if c2 < c1 {
                        let cp = clusters.remove(c1);
                        clusters[c2].extend(cp.iter());
                    } else if c1 < c2 {
                        let cp = clusters.remove(c2);
                        clusters[c1].extend(cp.iter());
                    }
                }
            },
        }
    }
    clusters.sort_by_key(|a| a.len());
    Some(
        clusters
            .iter()
            .rev()
            .take(3)
            .map(|x| x.len())
            .product::<usize>() as u64,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_internal(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|x| {
            let spl = x
                .split(",")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (spl[0], spl[1], spl[2])
        })
        .collect();
    let mut distances: Vec<(i64, usize, usize)> = vec![];
    for (i, b1) in boxes.iter().enumerate() {
        for (j, b2) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((dist(b1, b2), i, j));
        }
    }
    distances.sort_by_key(|a| a.0);

    let mut clusters: Vec<Box<Vec<usize>>> = vec![];
    for dist in distances.iter() {
        let fc1 = find_in_clusters(dist.1, &clusters);
        let fc2 = find_in_clusters(dist.2, &clusters);
        match fc1 {
            None => match fc2 {
                None => {
                    clusters.push(Box::new(vec![dist.1, dist.2]));
                }
                Some(c2) => clusters[c2].push(dist.1),
            },
            Some(c1) => match fc2 {
                None => {
                    clusters[c1].push(dist.2);
                }
                Some(c2) => {
                    if c2 < c1 {
                        let cp = clusters.remove(c1);
                        clusters[c2].extend(cp.iter());
                    } else if c1 < c2 {
                        let cp = clusters.remove(c2);
                        clusters[c1].extend(cp.iter());
                    }
                }
            },
        }
        if clusters.len() == 1 && clusters.first().unwrap().len() == boxes.len() {
            return Some((boxes[dist.1].0 * boxes[dist.2].0) as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_internal(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
