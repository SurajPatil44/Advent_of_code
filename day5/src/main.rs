use bit_vec::BitVec;
use std::dbg;
use std::fs::File;
use std::io::Read;

struct SeatNumber {
    row: BitVec,
    col: BitVec,
}

impl Default for SeatNumber {
    fn default() -> SeatNumber {
        SeatNumber {
            row: BitVec::from_elem(7, false),
            col: BitVec::from_elem(3, false),
        }
    }
}

impl SeatNumber {
    fn from_string(string: &str) -> SeatNumber {
        let mut sn = SeatNumber::default();
        let row = &string[..7];
        let col = &string[7..10];

        for (ind, chr) in row.chars().enumerate() {
            match chr {
                'F' => sn.row.set(ind, false),
                'B' => sn.row.set(ind, true),
                _ => unreachable!(),
            };
        }

        for (ind, chr) in col.chars().enumerate() {
            match chr {
                'R' => sn.col.set(ind, true),
                'L' => sn.col.set(ind, false),
                _ => unreachable!(),
            };
        }

        sn
    }

    fn get_seat_num(&self) -> u32 {
        let row = self.row.to_bytes()[0] >> 1;
        let col = self.col.to_bytes()[0] >> 5;

        row as u32 * 8 + col as u32
    }
}

fn main() {
    //let sn = SeatNumber::from_string("FBFBBFFRLR");
    let mut input = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let mut result: Vec<u32> = contents
        .lines()
        .map(|line| SeatNumber::from_string(line))
        .map(|sn| sn.get_seat_num())
        .collect();
    result.sort();
    let mut lst = result[0];
    for num in result[1..].iter() {
        if num - lst == 2 {
            dbg!(num, lst);
        } else {
            lst = *num;
        }
    }
    // dbg!(result);
}
