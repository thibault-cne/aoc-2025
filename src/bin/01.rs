advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.lines().fold((50, 0), |mut acc, l| {
        assert!(l.len() >= 1);
        let str = l.as_bytes();
        let is_left = matches!(str[0], b'L');
        let rotations = unsafe { l.get_unchecked(1..) }.parse::<isize>().unwrap();

        acc.0 = if is_left {
            (acc.0 - rotations).rem_euclid(100)
        } else {
            (acc.0 + rotations).rem_euclid(100)
        };

        if acc.0 == 0 {
            acc.1 += 1;
        }

        acc
    });

    Some(res.1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input.lines().fold((50, 0), |mut acc, l| {
        assert!(l.len() >= 1);
        let str = l.as_bytes();
        let is_left = matches!(str[0], b'L');
        let rotations = unsafe { l.get_unchecked(1..) }.parse::<isize>().unwrap();

        let compute = if is_left {
            acc.0 - rotations
        } else {
            acc.0 + rotations
        };
        let mut rev = (compute / 100).abs() as u32;

        if acc.0 != 0 && compute <= 0 {
            rev += 1;
        }

        acc.0 = compute.rem_euclid(100);
        acc.1 += rev;

        acc
    });

    Some(res.1)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_multiple_rotations() {
        let result = part_two("L50\nR1000");
        assert_eq!(result, Some(11));
    }
}
