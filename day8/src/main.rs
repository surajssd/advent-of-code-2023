use regex::Regex;
use std::collections::HashMap;
use std::fs;

static INPUT_FILE: &str = "input";
// static INPUT_FILE: &str = "smallinput2";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let inp = Input::new(lines);

    let mut count: u32 = 0;
    let mut next_node = "AAA".to_string();

    loop {
        if next_node == "ZZZ" {
            break;
        }

        for c in inp.instruction.chars() {
            // let temp_nex_node = next_node.borrow_mut();
            match c {
                'L' => {
                    next_node = inp.nodes.get(&next_node).cloned().unwrap().left;
                    count += 1;
                }
                'R' => {
                    next_node = inp.nodes.get(&next_node).cloned().unwrap().right;
                    count += 1;
                }
                _ => panic!("unkown char found in instructions: {}", c),
            }
        }
    }

    println!("{}", count);
}

#[derive(Debug)]
struct Input {
    instruction: String,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

impl Input {
    fn new(lines: Vec<String>) -> Self {
        let mut ret = Input {
            instruction: String::from(""),
            nodes: HashMap::new(),
        };

        for (idx, line) in lines.iter().enumerate() {
            if line.trim().len() == 0 {
                continue;
            }

            if idx == 0 {
                ret.instruction = line.clone();
                continue;
            }

            // Regex
            // Parse the line: AAA = (BBB, CCC)
            let re = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();
            let parsed_val: (&str, [&str; 3]) = re.captures(line).map(|c| c.extract()).unwrap();

            ret.nodes.insert(
                parsed_val.1[0].to_string(),
                Node {
                    left: parsed_val.1[1].to_string(),
                    right: parsed_val.1[2].to_string(),
                },
            );
        }

        ret
    }
}
