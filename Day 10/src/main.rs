use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_distinct_arrangements(
    numbers: &HashSet<u64>,
    search_number: u64,
    current_number: u64,
    cached_arrangements: &mut HashMap<u64, u64>,
) -> u64 {
    let mut count: u64 = 0;
    if cached_arrangements.contains_key(&current_number) {
        return *cached_arrangements.get(&current_number).unwrap();
    }
    if current_number == search_number {
        cached_arrangements.insert(current_number, 1);
        return 1;
    }
    if numbers.contains(&current_number) {
        for delta in [1, 2, 3] {
            count += find_distinct_arrangements(
                numbers,
                search_number,
                current_number + delta,
                cached_arrangements,
            );
            if count > 1 {
                cached_arrangements.insert(current_number, count);
            }
        }
        count
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut numbers: Vec<u64> = lines.iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut numbers_hashed: HashSet<u64> = HashSet::new();

    let mut jolt_1: u64 = 0;
    let mut jolt_3: u64 = 0;

    let mut cached_arrangements: HashMap<u64, u64> = HashMap::new();

    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    for (index, number) in numbers.iter().enumerate().skip(1) {
        match number - numbers[index - 1] {
            1 => jolt_1 += 1,
            3 => jolt_3 += 1,
            _ => break,
        }
        numbers_hashed.insert(*number);
    }
    numbers_hashed.insert(0);

    println!("Product of count of jolt differences: {}", jolt_1 * jolt_3);
    println!(
        "Distinct Arrangements: {}",
        find_distinct_arrangements(
            &numbers_hashed,
            *numbers.last().unwrap(),
            0,
            &mut cached_arrangements
        )
    );

    Ok(())
}
