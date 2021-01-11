use circular_queue::CircularQueue;
#[allow(unused_imports)]
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum InstrKind {
    NOP,
    JMP,
    ACC,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    instr: InstrKind,
    operand: isize,
}

#[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone)]
struct Status {
    pc: usize,
    acc: isize,
}

impl Status {
    fn next(self, pgm: &[Instruction], cache: &mut CircularQueue<Instruction>) -> Self {
        let ins = pgm[self.pc];
        cache.push(ins);
        match ins.instr {
            //cache.push_back(ins);
            InstrKind::NOP => Self {
                pc: self.pc + 1,
                acc: self.acc,
            },
            InstrKind::JMP => Self {
                pc: (self.pc as isize + ins.operand).try_into().unwrap(),
                acc: self.acc,
            },
            InstrKind::ACC => Self {
                pc: self.pc + 1,
                acc: self.acc + ins.operand,
            },
        }
    }
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(' ');
            Instruction {
                instr: match tokens.next() {
                    Some(tok) => match tok {
                        "nop" => InstrKind::NOP,
                        "acc" => InstrKind::ACC,
                        "jmp" => InstrKind::JMP,
                        _ => panic!("unknown instructions {}", tok),
                    },
                    None => panic!("this {} has problem", line),
                },
                operand: match tokens.next() {
                    Some(tok) => tok.parse().unwrap(),
                    None => panic!("this {} has problem", line),
                },
            }
        })
        .collect()
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let pgm = parse_program(&buffer);
    /*
    let mut cache = CircularQueue::<Instruction>::with_capacity(5);
    let mut iter = itertools::iterate(Status::default(), |s| s.next(&pgm,&mut cache));
    let mut set: HashSet<usize> = Default::default();
    let answer = iter.find(|state| !set.insert(state.pc)).unwrap();
    dbg!(answer.pc,answer.acc);
    dbg!(cache.iter().take(5).collect::<Vec<_>>());
    */
    let mm = pgm
        .iter()
        .filter(|i| matches!(i.instr, InstrKind::JMP | InstrKind::NOP))
        .count();
    dbg!(mm);
}
