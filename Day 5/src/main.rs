use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use bitvec::prelude as bv;
use bitvec::prelude::BitField;

fn boarding_pass_find_id (pass: String) -> i32 {

    let mut b_pass_row = bv::bitvec![];
    let mut b_pass_col = bv::bitvec![];

    for character in pass.chars() {
        if character == 'F' {
            b_pass_row.push(false);
        } else if character == 'B' {
            b_pass_row.push(true);
        } else if character == 'R' {
            b_pass_col.push(true);
        } else if character == 'L' {
            b_pass_col.push(false);
        }
    }

    b_pass_row.reverse();
    b_pass_col.reverse();

    let row: u8 = b_pass_row[0..7].load::<u8>();
    let col: u8 = b_pass_col[0..3].load::<u8>();
    let id: i32 = (row as i32 * 8) + col as i32;

    return id;
}

fn main() -> io::Result<()> {

    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);

    let mut prev_id: i32 = 0;
    let mut id_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        id_list.push(boarding_pass_find_id(line));
    }

    id_list.sort();
    for id in id_list.iter() {
        if id - prev_id == 2 {
            println!("Missing ID: {}", id - 1);
        }
        prev_id = *id;
    }

    println!("Highest ID: {}", prev_id);

    Ok(())
}
