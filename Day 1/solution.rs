use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let number_string: String = line.unwrap();
        let number: i32 = number_string.parse().unwrap();
        numbers.push(number);
        for &num1 in &numbers {
            if num1 == (2020 - number) {
                println!("{}", num1 * number);
            }
        }
    }

    for &num1 in &numbers {
        for &num2 in &numbers {
            for &num3 in &numbers {
                if num3 == (2020 - (num1 + num2)) {
                    println!("{}", num1 * num2 * num3);
                    break;
                }
            }
        }
    }

    Ok(())
}
