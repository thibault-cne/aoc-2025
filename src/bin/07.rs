use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let line_size = input.find('\n').unwrap();

    Some(
        input
            .lines()
            .fold(
                (HashSet::with_capacity(line_size), 0),
                |(mut acc, mut cpt), l| {
                    if let Some(index) = l.find('S') {
                        acc.insert(index);
                    } else {
                        let bytes = l.as_bytes();
                        acc.extract_if(|&e| bytes[e] == b'^')
                            .flat_map(|e| {
                                cpt += 1;
                                let mut tmp = Vec::with_capacity(2);
                                if e > 0 {
                                    tmp.push(e - 1);
                                }
                                if e < line_size {
                                    tmp.push(e + 1);
                                }
                                tmp.into_iter()
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                            .for_each(|e| {
                                acc.insert(e);
                            });
                    }

                    (acc, cpt)
                },
            )
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let line_size = input.find('\n').unwrap();

    Some(
        input
            .lines()
            .fold(vec![0; line_size], |mut acc, l| {
                if let Some(index) = l.find('S') {
                    acc[index] = 1;
                } else {
                    l.bytes()
                        .enumerate()
                        .filter(|&(_, b)| b == b'^')
                        .for_each(|(i, _)| {
                            let curr = acc[i];
                            if let Some(val) = acc.get_mut(i - 1) {
                                *val += curr;
                            }

                            if let Some(val) = acc.get_mut(i + 1) {
                                *val += curr;
                            }
                            acc[i] = 0;
                        });
                }

                acc
            })
            .into_iter()
            .sum(),
    )
}

#[cfg(not(feature = "test_lib"))]
#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(40));
    }
}
