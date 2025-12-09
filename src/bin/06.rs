use std::{
    iter::Sum,
    ops::{AddAssign, MulAssign},
};

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Op<T> {
    Add(T),
    Mul(T),
}

impl<T: AddAssign<T> + MulAssign<T>> Op<T> {
    fn update(&mut self, val: T) {
        match self {
            Op::Add(ref mut i) => *i += val,
            Op::Mul(ref mut i) => *i *= val,
        }
    }
}

impl From<Op<u64>> for u64 {
    fn from(value: Op<u64>) -> Self {
        match value {
            Op::Add(i) => i,
            Op::Mul(i) => i,
        }
    }
}

impl Sum<Op<u64>> for u64 {
    fn sum<I: Iterator<Item = Op<u64>>>(iter: I) -> Self {
        iter.map(Into::<u64>::into).sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let size = input
        .lines()
        .next()
        .map(|l| l.split_whitespace().count())
        .unwrap();

    let res = input
        .lines()
        .rev()
        .map(|l| l.split_whitespace())
        .enumerate()
        .fold(Vec::with_capacity(size), |mut acc, (i, val)| {
            if i == 0 {
                val.for_each(|val| match val {
                    "+" => acc.push(Op::Add(0)),
                    "*" => acc.push(Op::Mul(1)),
                    _ => unreachable!(),
                });
            } else {
                val.enumerate().for_each(|(j, val)| {
                    let val = val.parse::<u64>().unwrap();
                    acc[j].update(val);
                });
            }

            acc
        })
        .into_iter()
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let bytes = input.as_bytes();
    let line_size = input.find('\n').unwrap() + 1;
    let lines_count = input.lines().count() - 1;

    let mut operators: Vec<Op<u64>> = input
        .lines()
        .rev()
        .take(1)
        .flat_map(|l| {
            l.split_whitespace().map(|op| match op {
                "+" => Op::Add(0),
                "*" => Op::Mul(1),
                _ => unreachable!(),
            })
        })
        .collect();

    let mut index = 0;

    (0..line_size).filter(|&i| bytes[i] != b'\n').for_each(|j| {
        let value = (0..lines_count).fold(None, |acc, i| {
            if bytes[i * line_size + j] != b' ' {
                match acc {
                    Some(acc) => Some(acc * 10 + bytes[i * line_size + j] as u64 - 48),
                    None => Some(bytes[i * line_size + j] as u64 - 48),
                }
            } else {
                acc
            }
        });

        match value {
            Some(val) => operators[index].update(val),
            None => index += 1,
        }
    });

    Some(operators.into_iter().sum())
}

#[cfg(not(feature = "test_lib"))]
#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(3263827));
    }
}
