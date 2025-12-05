use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let line_len = input.lines().next().map(|l| l.len()).unwrap();
    let bytes: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();

    let res = (0..bytes.len() / line_len).fold(0, |acc, i| {
        acc + (0..line_len)
            .filter(|j| bytes[i * line_len + j] == b'@')
            .fold(0, |acc, j| {
                let mut cpt = 0;

                if i > 0 {
                    if j > 0 {
                        cpt += if bytes[(i - 1) * line_len + j - 1] == b'@' {
                            1
                        } else {
                            0
                        };
                    }
                    cpt += if bytes[(i - 1) * line_len + j] == b'@' {
                        1
                    } else {
                        0
                    };

                    if j < line_len - 1 {
                        cpt += if bytes[(i - 1) * line_len + j + 1] == b'@' {
                            1
                        } else {
                            0
                        };
                    }
                }

                if j > 0 {
                    cpt += if bytes[i * line_len + j - 1] == b'@' {
                        1
                    } else {
                        0
                    };
                }
                if j < line_len - 1 {
                    cpt += if bytes[i * line_len + j + 1] == b'@' {
                        1
                    } else {
                        0
                    };
                }

                if i < bytes.len() / line_len - 1 {
                    if j > 0 {
                        cpt += if bytes[(i + 1) * line_len + j - 1] == b'@' {
                            1
                        } else {
                            0
                        };
                    }
                    cpt += if bytes[(i + 1) * line_len + j] == b'@' {
                        1
                    } else {
                        0
                    };

                    if j < line_len - 1 {
                        cpt += if bytes[(i + 1) * line_len + j + 1] == b'@' {
                            1
                        } else {
                            0
                        };
                    }
                }

                if cpt < 4 {
                    acc + 1
                } else {
                    acc
                }
            })
    });

    Some(res)
}

struct State {
    neighbors: HashMap<usize, HashSet<usize>>,
}

impl State {
    fn compute(&mut self) -> u32 {
        self.neighbors.clone().iter().fold(0, |acc, (&cell, n)| {
            if n.len() < 4 {
                acc + self.remove_cell(cell)
            } else {
                acc
            }
        })
    }

    fn add_cell(&mut self, cell: usize, neighbors: HashSet<usize>) {
        self.neighbors.entry(cell).insert_entry(neighbors);
    }

    fn remove_cell(&mut self, cell: usize) -> u32 {
        if let Some(neighbors) = self.neighbors.remove(&cell) {
            assert!(neighbors.len() < 4);
            neighbors.iter().for_each(|&n| {
                self.neighbors.entry(n).and_modify(|e| {
                    e.remove(&cell);
                });
            });
            neighbors.iter().fold(1, |acc, &n| {
                let entry_len = self.neighbors.get(&n).map(|e| e.len()).unwrap_or(5);
                if entry_len < 4 {
                    acc + self.remove_cell(n)
                } else {
                    acc
                }
            })
        } else {
            0
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_len = input.lines().next().map(|l| l.len()).unwrap();
    let bytes: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();

    let mut state = State {
        neighbors: HashMap::new(),
    };
    (0..bytes.len() / line_len).for_each(|i| {
        (0..line_len)
            .filter(|j| bytes[i * line_len + j] == b'@')
            .for_each(|j| {
                let mut indexes = HashSet::with_capacity(8);

                if i > 0 {
                    if j > 0 && bytes[(i - 1) * line_len + j - 1] == b'@' {
                        indexes.insert((i - 1) * line_len + j - 1);
                    }
                    if bytes[(i - 1) * line_len + j] == b'@' {
                        indexes.insert((i - 1) * line_len + j);
                    }
                    if j < line_len - 1 && bytes[(i - 1) * line_len + j + 1] == b'@' {
                        indexes.insert((i - 1) * line_len + j + 1);
                    }
                }

                if j > 0 && bytes[i * line_len + j - 1] == b'@' {
                    indexes.insert(i * line_len + j - 1);
                }
                if j < line_len - 1 && bytes[i * line_len + j + 1] == b'@' {
                    indexes.insert(i * line_len + j + 1);
                }

                if i < bytes.len() / line_len - 1 {
                    if j > 0 && bytes[(i + 1) * line_len + j - 1] == b'@' {
                        indexes.insert((i + 1) * line_len + j - 1);
                    }
                    if bytes[(i + 1) * line_len + j] == b'@' {
                        indexes.insert((i + 1) * line_len + j);
                    }
                    if j < line_len - 1 && bytes[(i + 1) * line_len + j + 1] == b'@' {
                        indexes.insert((i + 1) * line_len + j + 1);
                    }
                }

                state.add_cell(i * line_len + j, indexes);
            })
    });

    let res = state.compute();

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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_example(DAY, PART_TWO));
        assert_eq!(result, Some(43));
    }
}
