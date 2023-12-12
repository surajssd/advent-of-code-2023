use std::fs;

static INPUT_FILE: &str = "input";
// static INPUT_FILE: &str = "smallinput";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut input: Vec<Vec<i32>> = Vec::new();
    for line in lines.iter() {
        let nums: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        input.push(nums);
    }

    let mut sum = 0;
    for inp in input.iter() {
        sum += predict_future(inp);
    }

    println!("{}", sum);
}

fn predict_future(nums: &Vec<i32>) -> i32 {
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    diffs.push(nums.clone());

    let mut prev_nums = nums.clone();
    loop {
        let mut current_diff: Vec<i32> = Vec::new();

        for idx in 1..prev_nums.len() {
            current_diff.push(prev_nums[idx] - prev_nums[idx - 1]);
        }

        if all_zeros(&current_diff) {
            break;
        }

        diffs.push(current_diff.clone());
        prev_nums = current_diff;
    }

    let mut p: i32 = 0;
    for d in diffs.iter().rev() {
        p += d.last().unwrap();
    }

    p
}

fn all_zeros(nums: &Vec<i32>) -> bool {
    for n in nums.iter() {
        if *n != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_zeros() {
        assert_eq!(all_zeros(&vec![0, 0, 0, 3, 4]), false);
        assert_eq!(all_zeros(&vec![0, 0, 0, 0, 0, 0]), true);
    }

    #[test]
    fn test_predict_future() {
        assert_eq!(predict_future(&vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_future(&vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(predict_future(&vec![10, 13, 16, 21, 30, 45]), 68);
    }
}
