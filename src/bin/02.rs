advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input.trim().split(",").fold(0, |acc, range| {
        let (l, r) = range
            .split_once("-")
            .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
            .unwrap();

        acc + (l..=r).fold(0, |acc, id| {
            let id_str = id.to_string();
            let half = id_str.len() / 2;
            if id_str[..half] == id_str[half..] {
                acc + id
            } else {
                acc
            }
        })
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input.trim().split(",").fold(0, |acc, range| {
        let (l, r) = range
            .split_once("-")
            .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
            .unwrap();

        acc + (l..=r).fold(0, |acc, id| {
            let id_str = id.to_string();
            let half = id_str.len() / 2;
            acc + (0..half)
                .filter(|limit| id_str.len().rem_euclid(limit + 1) == 0)
                .find(|&limit| {
                    id_str[..=limit]
                        .chars()
                        .cycle()
                        .zip(id_str.chars())
                        .all(|(a, b)| a == b)
                })
                .map(|_| id)
                .unwrap_or_default()
        })
    });

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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(4174379265));
    }
}
