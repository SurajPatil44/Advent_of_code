use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn process_block(block: &str) -> usize {
    let mut set: HashSet<char> = block.lines().next().unwrap().chars().collect();
    for line in block.lines() {
        let iset: HashSet<char> = line.chars().collect();
        set = set.intersection(&iset).cloned().collect();
    }
    set.len()
}

fn main() {
    let mut input = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let sum : usize = contents
        .split("\r\n\r\n")
        .map(|line| process_block(line))
        .sum();
    dbg!(sum);
}
