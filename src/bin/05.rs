use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let ranges = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split_once("-")
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .unwrap()
        })
        .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
        .fold(Vec::new(), |mut acc, (l, r)| {
            if acc.is_empty() {
                acc.push((l, r));
                acc
            } else {
                let last = acc.len() - 1;

                if l >= acc[last].0 && l <= acc[last].1 {
                    acc[last].1 = std::cmp::max(r, acc[last].1);
                } else {
                    acc.push((l, r));
                }
                acc
            }
        });

    let res = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.parse::<usize>().unwrap())
        .fold(0, |acc, id| {
            if ranges.iter().any(|(l, r)| (l..=r).contains(&&id)) {
                acc + 1
            } else {
                acc
            }
        });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split_once("-")
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .unwrap()
        })
        .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
        .fold(Vec::new(), |mut acc, (l, r)| {
            if acc.is_empty() {
                acc.push((l, r));
                acc
            } else {
                let last = acc.len() - 1;

                if l >= acc[last].0 && l <= acc[last].1 {
                    acc[last].1 = std::cmp::max(r, acc[last].1);
                } else {
                    acc.push((l, r));
                }
                acc
            }
        })
        .iter()
        .map(|r| (r.0..=r.1).count() as u64)
        .sum::<u64>();

    Some(res)
}

#[cfg(not(feature = "test_lib"))]
#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(14));
    }
}
