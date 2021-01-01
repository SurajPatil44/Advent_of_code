use std::dbg;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Tile {
    Tree,
    Open,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::Open
    }
}

struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
}

#[allow(unused_variables)]
impl Map {
    fn get_from_file<P: AsRef<Path>>(fpath: P) -> Map {
        let mut file = File::open(fpath).expect("can't open file");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let ht = buffer.lines().count();
        let wd = buffer.lines().next().unwrap().chars().count();
        dbg!(ht, wd);
        let mut grid = Vec::<Tile>::with_capacity(wd * ht);
        for line in buffer.lines() {
            for chr in line.chars() {
                let tile = match chr {
                    '.' => Tile::Open,
                    '#' => Tile::Tree,
                    _ => unreachable!(),
                };
                grid.push(tile);
            }
        }
        let map = Map {
            grid: grid,
            width: wd,
            height: ht,
        };

        map
    }
    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        //dbg!(self.height, self.width);
        let mut y_new = y;
        if x >= self.height {
            None
        } else {
            if y >= self.width {
                //println!("here");
                y_new = y.rem_euclid(self.width);
            }
            //dbg!(y_new);
            Some(self.grid[x * self.width + y_new])
        }
    }
    fn get_tile(&self, x: usize, y: usize, count: &mut usize) {
        let tile = self.get(x, y).unwrap();
        //dbg!(x, y);
        match tile {
            Tile::Tree => {
                *count += 1;
                //dbg!(tile, count);
            }
            Tile::Open => (),
        };
    }

    fn get_count(&self, right: usize, down: usize) -> usize {
        let mut count = 0;
        //let mut j = 2;
        for i in (0..self.height).step_by(down) {
            let s = (i as f32 * (right as f32 / down as f32)) as usize;
            self.get_tile(i, s, &mut count);
        }
        dbg!(right, down);
        dbg!(count)
    }
}
fn main() {
    let map = Map::get_from_file("input.txt");
    assert_eq!(map.get(0, 30).unwrap(), Tile::Tree);
    assert_eq!(map.get(0, 33).unwrap(), map.get(0, 3).unwrap());
    assert_eq!(map.get(323, 0), Option::<Tile>::None);
    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut tot_count = 1;
    for step in steps {
        tot_count *= map.get_count(step.0, step.1);
    }
    dbg!(tot_count);
}
