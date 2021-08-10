use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_bag_in_bags(
    bags: &HashMap<String, String>,
    search_bag: &str,
    container: &mut Vec<String>,
) -> bool {
    // find the bag which contains the bag being searched for, then recursively search for the
    // found bag. push every found bag to the container vector

    for bag in bags.keys() {
        if bags.get(bag).unwrap().contains(search_bag) {
            find_bag_in_bags(bags, bag.as_str(), container);
            container.push(bag.to_string());
        }
    }

    true
}

fn count_bag_in_bags(bags: &HashMap<String, String>, search_bag: &str) -> i32 {
    // recursively count number of bags inside the search_bag

    let mut count: i32 = 0;

    let bag = bags.get(search_bag).unwrap();
    for content in bag.split(',') {
        //handle the case of a bag containing *n*o other bags
        if &content[0..1].to_string() == "n" {
            continue;
        }

        count += (content[0..1].parse::<i32>().unwrap())
            * (1 + count_bag_in_bags(bags, &content[1..].to_string()));
    }

    count
}

fn main() -> io::Result<()> {
    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);

    let mut bags: HashMap<String, String> = HashMap::new();
    let mut container: Vec<String> = Vec::new();

    let search_bag: &str = "shinygoldbag";

    for line in reader.lines() {
        let line = line.unwrap();
        bags.insert(
            line.split_whitespace()
                .skip(0)
                .take(3)
                .collect::<String>()
                .to_string()
                .replace("bags", "bag"),
            line.split_whitespace()
                .skip(4)
                .collect::<String>()
                .split('.')
                .next()
                .unwrap()
                .to_string()
                .replace("bags", "bag"),
        );
    }

    find_bag_in_bags(&bags, search_bag, &mut container);
    container.sort();
    container.dedup();
    println!(
        "Number of bags containing the required bag {}",
        container.len()
    );

    println!(
        "Number of bags inside the selected bag {}",
        count_bag_in_bags(&bags, search_bag)
    );

    Ok(())
}
