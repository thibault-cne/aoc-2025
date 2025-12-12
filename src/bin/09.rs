use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<_> = input
        .lines()
        .map(|l| {
            let mut elems = l.split(",").map(|e| e.parse::<i32>().unwrap());
            let x = elems.next().unwrap();
            let y = elems.next().unwrap();
            (x, y)
        })
        .collect();

    points
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            points
                .iter()
                .enumerate()
                .filter(|&(j, _)| i > j)
                .map(|(_, (x2, y2))| ((x1 - x2).abs() + 1) as u64 * ((y1 - y2).abs() + 1) as u64)
                .collect::<Vec<_>>()
        })
        .sorted_unstable()
        .rev()
        .next()
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<_> = input
        .lines()
        .map(|l| {
            let mut elems = l.split(",").map(|e| e.parse::<i32>().unwrap());
            let x = elems.next().unwrap();
            let y = elems.next().unwrap();
            (x, y)
        })
        .collect();

    points
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            points
                .iter()
                .enumerate()
                .filter(|&(j, _)| i > j)
                .filter(|&(_, (x2, y2))| is_inside_polygon((*x1, *y1), (*x2, *y2), &points))
                .map(|(_, (x2, y2))| ((x1 - x2).abs() + 1) as u64 * ((y1 - y2).abs() + 1) as u64)
                .collect::<Vec<_>>()
        })
        .sorted_unstable()
        .rev()
        .next()
}

fn is_inside_polygon(p1: (i32, i32), p2: (i32, i32), polygon: &[(i32, i32)]) -> bool {
    polygon
        .iter()
        .circular_tuple_windows()
        .all(|(line_start, line_end)| {
            let left = p1.0.max(p2.0) <= line_start.0.min(line_end.0);
            let right = p1.0.min(p2.0) >= line_start.0.max(line_end.0);
            let above = p1.1.max(p2.1) <= line_start.1.min(line_end.1);
            let bellow = p1.1.min(p2.1) >= line_start.1.max(line_end.1);

            left || right || above || bellow
        })
}

#[cfg(not(feature = "test_lib"))]
#[cfg(test)]
mod tests {
    use super::*;
    advent_of_code::part!();

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_example(DAY, PART_ONE));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(24));
    }
}
