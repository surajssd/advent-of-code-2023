// Here is the source: https://adventofcode.com/2023/day/2#part2
use std::fs;

const INPUT_FILE: &str = "input";

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
        sum += find_cube_power(game);
    }

    println!("{}", sum);
}

fn find_cube_power(g: Game) -> u32 {
    // Go through all the sets in the game
    // Find max of each
    // multiply them and send it back

    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for set in g.sets.iter() {
        if set.red > red {
            red = set.red;
        }

        if set.green > green {
            green = set.green;
        }

        if set.blue > blue {
            blue = set.blue
        }
    }

    red * green * blue
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_find_cube_power() {
        let tests = [
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
                48,
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
                1560,
            ),
        ];

        for test in tests.iter() {
            assert_eq!(find_cube_power(test.0.clone()), test.1);
        }
    }
}
