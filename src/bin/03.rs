use std::ops::ControlFlow;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines().fold(0, |acc, line| {
        let bytes = line.as_bytes();

        let (max, max_2) = match bytes
            .iter()
            .enumerate()
            .try_fold((0, 0), |mut acc, (i, v)| {
                let digit = v - 48;

                if digit > bytes[acc.0] - 48 {
                    acc.1 = acc.0;
                    acc.0 = i;
                } else if digit > bytes[acc.1] - 48 {
                    acc.1 = i;
                }

                if bytes[acc.0] - 48 == 9 {
                    ControlFlow::Break(acc)
                } else {
                    ControlFlow::Continue(acc)
                }
            }) {
            ControlFlow::Continue(acc) => acc,
            ControlFlow::Break(acc) => acc,
        };

        let max_v = bytes[max] - 48;
        if max == bytes.len() - 1 {
            let max_2 = bytes[max_2] - 48;
            acc + (max_2 * 10 + max_v) as u32
        } else {
            let max_2 = bytes
                .iter()
                .skip(max + 1)
                .fold(0, |acc, v| if v - 48 > acc { v - 48 } else { acc });
            acc + (max_v * 10 + max_2) as u32
        }
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input.lines().fold(0, |acc, line| {
        let bytes = line.as_bytes();

        acc + (0..=11)
            .rev()
            .fold((0, 0), |(base_i, acc), i| {
                let (new_i, add_v) = bytes
                    .iter()
                    .enumerate()
                    .skip(base_i)
                    .take(bytes.len() - i - base_i)
                    .fold((0, 0), |acc, (i, &v)| if v > acc.1 { (i, v) } else { acc });

                let add_v = add_v as u64 - 48;

                (new_i + 1, acc * 10 + add_v)
            })
            .1
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(3121910778619));
    }
}
