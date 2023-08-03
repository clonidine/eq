use std::{io, num::ParseIntError};

struct Solver;

impl Solver {
    fn get_left(equation: &String) -> Result<Vec<i32>, ParseIntError> {
        let mut left = Vec::new();

        let parts: Vec<&str> = equation.split_whitespace().collect();

        let mut index = 0;

        for part in parts {
            match part {
                "=" => break,
                "+" | "x" => continue,
                _ => {
                    let number: Result<i32, ParseIntError> = part.parse();

                    match number {
                        Ok(number) => {
                            left.insert(index, number);

                            index += 1;
                        }

                        Err(_) => panic!("Invalid equation format."),
                    }
                }
            }
        }

        Ok(left)
    }

    fn get_right(equation: &String) -> Result<Vec<i32>, ParseIntError> {
        let mut right = Vec::new();

        let parts: Vec<&str> = equation.split_whitespace().collect();

        let mut index_1 = parts.len() - 1;
        let mut index_2 = 0;

        for _ in 0..parts.len() {
            let part = parts.get(index_1);

            match part {
                Some(part) => match part {
                    &"=" => break,
                    &"+" | &"x" => continue,

                    _ => {
                        let number: Result<i32, ParseIntError> = part.parse();

                        match number {
                            Ok(number) => {
                                right.insert(index_2, number);
                                index_2 += 1;
                            }

                            Err(_) => println!("Invalid equation format."),
                        }
                    }
                },

                None => break,
            }

            index_1 -= 1;
        }

        Ok(right)
    }

    fn solve(equation: &String) -> Result<i32, ParseIntError> {
        let left = Self::get_left(&equation);
        let right = Self::get_right(&equation);

        match left {
            Ok(left) => match right {
                Ok(right) => {
                    let left: i32 = left.iter().sum();
                    let right: i32 = right.iter().sum();

                    let x = right - left;

                    Ok(x)
                }

                Err(e) => panic!("{}", e),
            },

            Err(e) => panic!("{}", e),
        }
    }
}

fn main() {
    loop {
        let mut equation = String::new();

        io::stdin().read_line(&mut equation).unwrap();

        let x = Solver::solve(&equation);

        match x {
            Ok(x) => {
                println!("x = {}", x)
            }

            Err(e) => panic!("{}", e),
        }
    }
}
