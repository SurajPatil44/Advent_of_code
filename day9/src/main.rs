use itertools::Itertools;
use std::fs::File;
use std::io::Read;

const WINDOW_SIZE: usize = 25;

fn find(wins: &[u64], num: u64) -> (bool, Option<u64>) {
    let res = wins.iter().tuple_combinations().any(|(a, b)| a + b == num);
    dbg!(res, num);
    match res {
        true => (true, None),
        false => (false, Some(num)),
    }
}

fn main() {
    let mut input = File::open("input.txt").expect("file not found");
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).expect("failed to read");
    let nums: Vec<u64> = buffer
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .expect(format!("error is {}", line).as_ref())
        })
        .collect();
    for ind in 0..nums.len() - WINDOW_SIZE {
        let res = find(&nums[ind..(ind + WINDOW_SIZE)], nums[ind + WINDOW_SIZE]);
        if !res.0 {
            dbg!(res.1);
            break;
        } else {
        }
    }
}
