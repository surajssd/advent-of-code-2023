// Here is the source: https://adventofcode.com/2023/day/3#part2
// Here is the sample input: https://adventofcode.com/2023/day/3/input
use std::collections::HashMap;
use std::fmt;
use std::fs;

const INPUT_FILE: &str = "input";

fn main() {
    let scheme: Vec<Vec<char>> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let s = Schematic {
        scheme: scheme.clone(),
        max_row: scheme.len(),
        max_column: scheme[0].len(),
    };

    let mut i: usize = 0;
    let mut sum = 0;

    while i < s.max_row {
        let mut j: usize = 0;
        while j < s.max_column {
            // When you encounter *
            if s.scheme[i][j] != '*' {
                j += 1;
                continue;
            }

            let num = Number {
                digits: vec![Digit { row: i, column: j }],
                decimal: 0,
            };

            // Find the neighbors
            // Skip all the neighbors if they are not numeric
            // For the numeric neighbor, skip the *
            // --------
            // Remove duplicates because we might have entry for the same number twice.
            let mut map: HashMap<String, Number> = HashMap::new();

            for val in s.find_complete_numbers(s.neighbors(&num)) {
                let k = format!("{}", val);
                let _ = map.insert(k, val);
            }

            if map.len() <= 1 {
                j += 1;
                continue;
            }

            let mut mult = 1;
            for (_, v) in map.iter() {
                let mut b = v.clone();
                b.digits_to_decimal(&s);
                mult *= b.decimal;
            }

            sum += mult;

            j += 1;
        }
        i += 1;
    }

    println!("{}", sum);
}

#[derive(Debug)]
struct Schematic {
    scheme: Vec<Vec<char>>,
    max_row: usize,
    max_column: usize,
}

impl Schematic {
    fn value_at(&self, d: &Digit) -> char {
        self.scheme[d.row][d.column]
    }

    fn neighbors(&self, n: &Number) -> Vec<Digit> {
        let mut neighbors: Vec<Digit> = Vec::new();

        for digit in n.digits.iter() {
            // Lower Right Corner
            // r+1, c+1
            match self.next_row(digit.row) {
                None => None,
                Some(r) => match self.next_column(digit.column) {
                    None => None,
                    Some(c) => {
                        let d = Digit { row: r, column: c };
                        Some(self.filter_neighbors(&mut neighbors, d))
                    }
                },
            };

            // Upper Right Corner
            // r-1, c+1
            match self.prev_row(digit.row) {
                None => None,
                Some(r) => match self.next_column(digit.column) {
                    None => None,
                    Some(c) => {
                        let d = Digit { row: r, column: c };
                        Some(self.filter_neighbors(&mut neighbors, d))
                    }
                },
            };

            // Lower Left Corner
            // r+1, c-1
            match self.next_row(digit.row) {
                None => None,
                Some(r) => match self.prev_column(digit.column) {
                    None => None,
                    Some(c) => {
                        let d = Digit { row: r, column: c };
                        Some(self.filter_neighbors(&mut neighbors, d))
                    }
                },
            };

            // Upper Left Corner
            // r-1, c-1
            match self.prev_row(digit.row) {
                None => None,
                Some(r) => match self.prev_column(digit.column) {
                    None => None,
                    Some(c) => {
                        let d = Digit { row: r, column: c };
                        Some(self.filter_neighbors(&mut neighbors, d))
                    }
                },
            };

            // Left
            match self.prev_column(digit.column) {
                None => None,
                Some(c) => {
                    let d = Digit {
                        row: digit.row,
                        column: c,
                    };
                    Some(self.filter_neighbors(&mut neighbors, d))
                }
            };

            // Right
            match self.next_column(digit.column) {
                None => None,
                Some(c) => {
                    let d = Digit {
                        row: digit.row,
                        column: c,
                    };
                    Some(self.filter_neighbors(&mut neighbors, d))
                }
            };

            // Up
            match self.prev_row(digit.row) {
                None => None,
                Some(r) => {
                    let d = Digit {
                        row: r,
                        column: digit.column,
                    };

                    Some(self.filter_neighbors(&mut neighbors, d))
                }
            };

            // Down
            match self.next_row(digit.row) {
                None => None,
                Some(r) => {
                    let d = Digit {
                        row: r,
                        column: digit.column,
                    };

                    Some(self.filter_neighbors(&mut neighbors, d))
                }
            };
        }

        neighbors
    }

    fn next_row(&self, r: usize) -> Option<usize> {
        if r + 1 < self.max_row {
            return Some(r + 1);
        }

        None
    }

    fn prev_row(&self, r: usize) -> Option<usize> {
        if r == 0 {
            return None;
        }

        Some(r - 1)
    }

    fn next_column(&self, c: usize) -> Option<usize> {
        if c + 1 < self.max_column {
            return Some(c + 1);
        }

        None
    }

    fn prev_column(&self, c: usize) -> Option<usize> {
        if c == 0 {
            return None;
        }

        Some(c - 1)
    }

    fn filter_neighbors(&self, neighbors: &mut Vec<Digit>, d: Digit) {
        let c = self.value_at(&d);
        if !c.is_numeric() {
            return;
        }

        neighbors.push(d)
    }

    fn find_complete_numbers(&self, neighbors: Vec<Digit>) -> Vec<Number> {
        let mut numbers = Vec::new();

        for num in neighbors.iter() {
            // Go left until you find the . non-numeric char
            // Then go right until you find the . or non-numeric char
            let mut lhs = num.column;
            loop {
                match self.prev_column(lhs) {
                    None => break,
                    Some(new_lhs) => {
                        if !self.scheme[num.row][new_lhs].is_numeric() {
                            break;
                        }

                        lhs = new_lhs;
                    }
                };
            }

            let mut rhs = num.column;
            loop {
                match self.next_column(rhs) {
                    None => break,
                    Some(new_rhs) => {
                        if !self.scheme[num.row][new_rhs].is_numeric() {
                            break;
                        }

                        rhs = new_rhs;
                    }
                };
            }

            let mut new_num = Number {
                digits: Vec::new(),
                decimal: 0,
            };

            while lhs <= rhs {
                new_num.digits.push(Digit {
                    row: num.row,
                    column: lhs,
                });
                lhs += 1;
            }

            numbers.push(new_num);
        }

        numbers
    }
}

#[derive(Debug, Copy, Clone)]
struct Digit {
    row: usize,
    column: usize,
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d[{}][{}]", self.row, self.column)
    }
}

#[derive(Debug, Clone)]
struct Number {
    // Co-ordinates of the digit
    // number 573 will be [5, 7, 3]
    // 0,4 0,5 0,6
    digits: Vec<Digit>,
    decimal: u32,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in self.digits.iter() {
            write!(f, "{}", d);
        }

        Ok(())
    }
}

impl Number {
    fn digits_to_decimal(&mut self, s: &Schematic) {
        let mut num = 0;
        for d in self.digits.iter() {
            num *= 10;
            num += s.scheme[d.row][d.column].to_digit(10).unwrap();
        }

        self.decimal = num;
    }
}

// A number consists of co-ordinates of each digit.
// We need a way to get all the items (coordinates) in the vicinity of a number
// If a special symbol other than the dot . or space or number exists then it is a special number
