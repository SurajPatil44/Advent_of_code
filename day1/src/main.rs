use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn part1(nums: &Vec<u32>) {
    for ind1 in 0..nums.len() - 1 {
        for ind2 in ind1..nums.len() {
            if nums[ind1] + nums[ind2] == 2020 {
                println!("{}", nums[ind1] * nums[ind2]);
                break;
            }
        }
    }
}

fn part2(nums: &Vec<u32>) {
    let (a, b, c) = nums
        .into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| *a + *b + *c == 2020)
        .unwrap();
    println!("{} {}", a + b + c, a * b * c);
}

fn main() {
    let mut input = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let nums: Vec<u32> = contents
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();
    part1(&nums);
    part2(&nums);
}
