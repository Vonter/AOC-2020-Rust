use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn xmas_sum_check(sum: i64, number_cache: &HashSet<i64>) -> i64 {
    for number in number_cache.iter() {
        if number_cache.contains(&(sum - number)) {
            return 0;
        }
    }
    sum
}

fn sum_numbers(numbers: &[String]) -> i64 {
    let mut sum: i64 = 0;
    for number in numbers.iter() {
        sum += number.parse::<i64>().unwrap();
    }
    sum
}

fn sum_extremes(numbers: &[String]) -> i64 {
    let mut numbers_sorted: Vec<String> = Vec::new();
    numbers_sorted.extend_from_slice(numbers);
    numbers_sorted.sort();
    let sum: i64 = numbers_sorted[0].parse::<i64>().unwrap()
        + numbers_sorted[numbers_sorted.len() - 1]
            .parse::<i64>()
            .unwrap();
    sum
}

fn find_contiguous_sum(sum: i64, numbers: &[String], index_l: usize, index_r: usize) -> bool {
    if index_l >= index_r || index_r > numbers.len() {
        return false;
    }
    let calculated_sum = sum_numbers(&numbers[index_l..index_r]);
    match calculated_sum.cmp(&sum) {
        Ordering::Equal => {
            println!(
                "Sum of smallest and largest numbers is {}",
                sum_extremes(&numbers[index_l..index_r])
            );
            return true;
        }
        Ordering::Less => {
            if find_contiguous_sum(sum, numbers, index_l, index_r + 1) {
                return true;
            }
            if find_contiguous_sum(sum, numbers, index_l - 1, index_r) {
                return true;
            }
        }
        Ordering::Greater => {
            if find_contiguous_sum(sum, numbers, index_l + 1, index_r - 1) {
                return true;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut number_cache: HashSet<i64> = HashSet::new();
    let mut sum: i64;

    for (index, line) in lines.iter().enumerate() {
        sum = line.parse::<i64>().unwrap();
        number_cache.insert(sum);
        if number_cache.len() > 25 {
            sum = xmas_sum_check(sum, &number_cache);
            if sum != 0 {
                println!("{} is the invalid number.", sum);
                find_contiguous_sum(sum, &lines, 0, index);
                break;
            }
            number_cache.remove(&lines[index - 25].parse::<i64>().unwrap());
        }
    }

    Ok(())
}
