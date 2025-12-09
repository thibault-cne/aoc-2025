use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let points: Vec<_> = input
        .lines()
        .map(|l| {
            let mut elems = l.split(",").map(|e| e.parse::<f32>().unwrap());
            let x = elems.next().unwrap();
            let y = elems.next().unwrap();
            let z = elems.next().unwrap();
            (x, y, z)
        })
        .collect();

    let res = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            points
                .iter()
                .enumerate()
                .filter(|&(j, _)| i > j)
                .map(|(j, b)| {
                    (
                        ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt(),
                        (i, j),
                    )
                })
                .collect::<Vec<_>>()
        })
        .sorted_unstable_by(|a, b| a.0.total_cmp(&b.0))
        .take(1000)
        .fold(Vec::<HashSet<usize>>::new(), |mut acc, (_, (i, j))| {
            let l = acc.iter().position(|e| e.contains(&i));
            let r = acc.iter().position(|e| e.contains(&j));

            match (l, r) {
                (Some(l), Some(r)) if l != r => {
                    let right_map = std::mem::take(&mut acc[r]);
                    right_map.iter().for_each(|&e| {
                        acc[l].insert(e);
                    });
                    acc.swap_remove(r);
                }
                (Some(l), Some(r)) if l == r => (),
                (Some(e), _) | (_, Some(e)) => {
                    acc[e].insert(i);
                    acc[e].insert(j);
                }
                _ => {
                    let new_entry = HashSet::from([i, j]);
                    acc.push(new_entry);
                }
            }

            acc
        })
        .into_iter()
        .sorted_unstable_by(|a, b| b.len().cmp(&a.len()))
        .take(3)
        .map(|a| a.len())
        .product::<usize>();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<_> = input
        .lines()
        .map(|l| {
            let mut elems = l.split(",").map(|e| e.parse::<f64>().unwrap());
            let x = elems.next().unwrap();
            let y = elems.next().unwrap();
            let z = elems.next().unwrap();
            (x, y, z)
        })
        .collect();

    let mut iter = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            points
                .iter()
                .enumerate()
                .filter(|&(j, _)| i > j)
                .map(|(j, b)| {
                    (
                        ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt(),
                        (i, j),
                    )
                })
                .collect::<Vec<_>>()
        })
        .sorted_unstable_by(|a, b| a.0.total_cmp(&b.0));

    let mut acc = Vec::<HashSet<usize>>::new();

    loop {
        let (_, (i, j)) = iter.next().unwrap();
        let l = acc.iter().position(|e| e.contains(&i));
        let r = acc.iter().position(|e| e.contains(&j));

        match (l, r) {
            (Some(l), Some(r)) if l != r => {
                let right_map = std::mem::take(&mut acc[r]);
                right_map.iter().for_each(|&e| {
                    acc[l].insert(e);
                });
                acc.swap_remove(r);
            }
            (Some(l), Some(r)) if l == r => (),
            (Some(e), _) | (_, Some(e)) => {
                acc[e].insert(i);
                acc[e].insert(j);
            }
            _ => {
                let new_entry = HashSet::from([i, j]);
                acc.push(new_entry);
            }
        }

        if acc[0].len() == points.len() {
            return Some((points[i].0 * points[j].0) as u64);
        }
    }
}

#[cfg(not(feature = "test_lib"))]
#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(25272));
    }
}
