use std::{collections::HashSet, io::prelude::*, ptr::hash};
use std::{fs::File, io};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let readers= io::BufReader::new(file);
    let mut hash_set: HashSet<i64> = HashSet::new();
    for line in readers.lines() {
        hash_set.insert(line.unwrap().parse::<i64>().unwrap());
    } 

    for num in &hash_set {
        let mut target = 2020 - num;
        for num2 in &hash_set {
            if num == num2 {
                continue;
            }
            if hash_set.contains(&(target - num2)) {
                println!("{}", num * num2 * (target - num2));
            }
        }
    }
}
