// Here is the source: https://adventofcode.com/2023/day/2
// Here is the sample input: https://adventofcode.com/2023/day/2/input
use std::fs;

const INPUT_FILE: &str = "input";

const TOTAL_RED: u32 = 12;
const TOTAL_GREEN: u32 = 13;
const TOTAL_BLUE: u32 = 14;

#[derive(Clone, PartialEq, Debug, Copy)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Clone, PartialEq, Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn main() {
    // Read the file
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum: u32 = 0;

    for line in lines.iter() {
        let game = str_to_game(line.to_string());
        let id = game.id;

        if is_game_possible(game) {
            sum += id;
        }
    }

    println!("{}", sum);
}

fn str_to_game(line: String) -> Game {
    let game_and_sets: Vec<&str> = line.split(":").collect();

    if game_and_sets.len() < 2 {
        panic!("improper format of game string");
    }

    // Parse the game id out of string.
    let id = game_and_sets[0]
        .replace("Game", "")
        .trim()
        .parse::<u32>()
        .unwrap();

    // Parse the sets
    // Here convert the following
    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // To
    // ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
    let sets_str: Vec<&str> = game_and_sets[1].split(";").collect();
    let mut sets: Vec<Set> = Vec::new();

    for (idx, set_str) in sets_str.iter().enumerate() {
        // Parse a set from string
        // Here conver the following
        // 1 red, 2 green, 6 blue
        // To
        // ["1 red", "2 green", "6 blue"]
        let cubes: Vec<&str> = set_str.split(",").collect();

        let mut set = Set {
            red: 0,
            blue: 0,
            green: 0,
        };

        for cube in cubes.iter() {
            // Parse the color
            // 1 red
            let count_and_color: Vec<&str> = cube.trim().split(" ").collect();

            if count_and_color.len() != 2 {
                panic!(
                    "found badly formatted color: {} the length is {}",
                    cube,
                    cube.len()
                );
            }

            let count = count_and_color[0].parse::<u32>().unwrap();
            match count_and_color[1] {
                "red" => set.red = count,
                "blue" => set.blue = count,
                "green" => set.green = count,
                _ => panic!("found unknown color: {}", count_and_color[1]),
            };
        }

        sets.push(set);
    }

    return Game { id: id, sets: sets };
}

fn is_game_possible(g: Game) -> bool {
    for s in g.sets.iter() {
        if !is_set_possible(*s) {
            return false;
        }
    }

    true
}

fn is_set_possible(set: Set) -> bool {
    set.red <= TOTAL_RED && set.green <= TOTAL_GREEN && set.blue <= TOTAL_BLUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_set_possible() {
        let sets = [
            (
                Set {
                    red: 4,
                    blue: 0,
                    green: 3,
                },
                true,
            ),
            (
                Set {
                    red: 20,
                    blue: 8,
                    green: 6,
                },
                false,
            ),
            (
                Set {
                    red: 14,
                    blue: 3,
                    green: 15,
                },
                false,
            ),
            (
                Set {
                    red: 6,
                    blue: 3,
                    green: 1,
                },
                true,
            ),
        ];

        for set in sets.iter() {
            assert_eq!(is_set_possible(set.0), set.1);
        }
    }

    #[test]
    fn test_is_game_possible() {
        let games = [
            (
                Game {
                    id: 1,
                    sets: vec![
                        Set {
                            blue: 3,
                            red: 4,
                            green: 0,
                        },
                        Set {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Set {
                            green: 2,
                            red: 0,
                            blue: 0,
                        },
                    ],
                },
                true,
            ),
            (
                Game {
                    id: 3,
                    sets: vec![
                        Set {
                            green: 8,
                            blue: 6,
                            red: 20,
                        },
                        Set {
                            blue: 5,
                            red: 4,
                            green: 13,
                        },
                        Set {
                            green: 5,
                            red: 1,
                            blue: 0,
                        },
                    ],
                },
                false,
            ),
        ];

        for game in games.iter() {
            assert_eq!(is_game_possible(game.0.clone()), game.1);
        }
    }

    #[test]
    fn test_str_to_game() {
        let inputs = [
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                Game {
                    id: 1,
                    sets: vec![
                        Set {
                            blue: 3,
                            red: 4,
                            green: 0,
                        },
                        Set {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Set {
                            green: 2,
                            red: 0,
                            blue: 0,
                        },
                    ],
                },
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                Game {
                    id: 3,
                    sets: vec![
                        Set {
                            green: 8,
                            blue: 6,
                            red: 20,
                        },
                        Set {
                            blue: 5,
                            red: 4,
                            green: 13,
                        },
                        Set {
                            green: 5,
                            red: 1,
                            blue: 0,
                        },
                    ],
                },
            ),
        ];

        for input in inputs.iter() {
            assert_eq!(str_to_game(String::from(input.0)), input.1);
        }
    }
}
