use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn tobbogan (map: Vec<Vec<i64>>, trees: i64, slope: [usize; 2], mut pos: [usize; 2]) -> i64 {

    pos = [pos[0] + slope[0], slope[1]];

    if pos[1] >= map.len() {
        return trees;
    }
    if pos[0] >= map[0].len() {
        pos[0] = pos[0] % map[0].len();
    }

    if map[pos[1]][pos[0]] == 1 {
        return tobbogan(map[slope[1]..].to_vec(), trees + 1, slope, pos);
    } else {
        return tobbogan(map[slope[1]..].to_vec(), trees, slope, pos);
    }

}

fn main() -> io::Result<()> {

    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<i64>> = vec![vec![]];

    for line in reader.lines() {
        let mut row: Vec<i64> = vec![];

        for character in line.unwrap().chars() {
            if character == '.' {
                row.push(0);
            } else {
                row.push(1);
            }
        }

        map.push(row);
    }

    println!("{}", tobbogan(map[1..].to_vec(), 0, [3, 1], [0, 0]));
    println!("{}", tobbogan(map[1..].to_vec(), 0, [1, 1], [0, 0])
                    * tobbogan(map[1..].to_vec(), 0, [3, 1], [0, 0])
                    * tobbogan(map[1..].to_vec(), 0, [5, 1], [0, 0])
                    * tobbogan(map[1..].to_vec(), 0, [7, 1], [0, 0])
                    * tobbogan(map[1..].to_vec(), 0, [1, 2], [0, 0]));

    Ok(())
}
