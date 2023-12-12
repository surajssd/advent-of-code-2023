use regex::Regex;
use std::collections::HashMap;
use std::fs;

// static INPUT_FILE: &str = "input";
static INPUT_FILE: &str = "smallinput2";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let inp = Input::new(lines);

    let mut count: u32 = 0;
    // Populate starting points
    // Find all the nodes that end with A these will become the starting points
    let mut next_nodes: Vec<String> = starting_nodes(&inp);
    let mut round_trip = vec![0; next_nodes.len()];
    let mut break_loop = false;

    let mut start_nodes = vec!["".to_string(); next_nodes.len()];
    let c = inp.instruction.chars().next().unwrap();
    for (idx, nn) in next_nodes.iter_mut().enumerate() {
        match c {
            'L' => {
                start_nodes[idx] = inp.nodes.get(nn).cloned().unwrap().left;
            }
            'R' => {
                start_nodes[idx] = inp.nodes.get(nn).cloned().unwrap().right;
            }
            _ => panic!("unkown char found in instructions: {}", c),
        };
    }

    while !break_loop {
        for c in inp.instruction.chars() {
            // let mut found_non_z = false;

            // Add for loop for each of the steps
            for (idx, nn) in next_nodes.iter_mut().enumerate() {
                match c {
                    'L' => {
                        *nn = inp.nodes.get(nn).cloned().unwrap().left;
                    }
                    'R' => {
                        *nn = inp.nodes.get(nn).cloned().unwrap().right;
                    }
                    _ => panic!("unkown char found in instructions: {}", c),
                }

                // println!("*nn: {}, start_nodes[{}]: {}", *nn, idx, start_nodes[idx]);
                if *nn == start_nodes[idx] {
                    if round_trip[idx] == 0 {
                        round_trip[idx] = count;
                    }
                }

                // // If we find any string that does not end with z we don't consider this batch and move forward.
                // if !is_z(nn) {
                //     found_non_z = true;
                // }
            }

            // println!("{:?}", next_nodes);

            count += 1;

            if all_non_zero(&round_trip) {
                break_loop = true;
                break;
            }

            // // When we don't find any string that don't end with z
            // // As in all the strings end with z then we break.
            // if !found_non_z {
            //     break_loop = true;
            //     break;
            // }
        }
    }

    println!("{:?}", round_trip);

    println!("{}", lcm(round_trip));
}

fn lcm(nums: Vec<u32>) -> u64 {
    let mut ans: u64 = 0;

    for (idx, num) in nums.iter().enumerate() {
        let num_big = u64::from(*num);
        if idx == 0 {
            ans = num_big;
            continue;
        }

        ans = num_big * ans / gcd(num_big, ans);
    }

    ans
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn all_non_zero(counts: &Vec<u32>) -> bool {
    for cnt in counts.iter() {
        if *cnt == 0 {
            return false;
        }
    }

    true
}

fn starting_nodes(inp: &Input) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for (k, _) in &inp.nodes {
        if is_a(k) {
            ret.push(k.to_string());
        }
    }

    ret
}

fn is_a(s: &String) -> bool {
    s.chars().last().unwrap() == 'A'
}

// fn is_z(s: &String) -> bool {
//     s.chars().last().unwrap() == 'Z'
// }

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
