use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Element<'a> {
    range: (usize, usize),
    password: &'a str,
    token: char,
}

impl<'a> Element<'_> {
    fn from_str(inp: &str) -> Element {
        let parts: Vec<&str> = inp.split(' ').collect();
        //println!("{:#?}",parts);
        let range: Vec<usize> = parts[0]
            .split('-')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect();
        let range = (range[0], range[1]);
        let token = parts[1].chars().take(1).next().unwrap();
        let password = parts[2];
        Element {
            range,
            password,
            token,
        }
    }

    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.token).count();
        return count >= self.range.0 && count <= self.range.1;
    }

    fn is_valid_2(&self) -> bool {
        let first = self.password.chars().nth(self.range.0 - 1).unwrap() == self.token;
        let second = self.password.chars().nth(self.range.1 - 1).unwrap() == self.token;
        first ^ second
    }
}

fn main() {
    let mut input = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    //part 1
    let count = contents
        .lines()
        .map(|line| Element::from_str(line))
        .filter(|el| el.is_valid())
        .count();
    println!("{}", count);
    //part 2
    let count = contents
        .lines()
        .map(|line| Element::from_str(line))
        .filter(|el| el.is_valid_2())
        .count();
    println!("{}", count);
}
