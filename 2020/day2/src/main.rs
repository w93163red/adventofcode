use std::{collections::HashMap, fs::File, io, io::prelude::*};
use io::Lines;
use anyhow::Result;

fn main() -> Result<()> {
    part_1();
    part_2();
    Ok(())
}

fn read_lines() -> Lines<io::BufReader<File>> {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let readers= io::BufReader::new(file);
    readers.lines()
}

fn part_1() -> Result<()> {
    let lines = read_lines();
    let mut ans = 0;
    for line in lines {
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        let line = line?;
        let split: Vec<_> = line.split_whitespace().collect(); 
        // get the range
        let split_num: Vec<_> = split[0].split('-').collect();
        let min: i32 = split_num[0].parse()?;
        let max: i32 = split_num[1].parse()?;
        let letter = split[1].chars().nth(0).unwrap();
        for c in split[2].chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        match char_counts.get(&letter) {
            Some(count) => {
                if count >= &min && count <= &max {
                    ans += 1
                }
            },
            _ => {}
        } 
     }
    println!("{}", ans);
    Ok(())
}

fn part_2() -> Result<()> {
    let lines = read_lines();
    let mut ans = 0;
    for line in lines {
        let line = line?;
        let split: Vec<_> = line.split_whitespace().collect(); 
        // get the range
        let split_num: Vec<_> = split[0].split('-').collect();
        let first: usize = split_num[0].parse()?;
        let second: usize = split_num[1].parse()?;
        let letter = split[1].chars().nth(0).unwrap();
        let target = split[2];
        let is_first = target.chars().nth(first - 1) == Some(letter);
        let is_second = target.chars().nth(second - 1) == Some(letter);
        if is_first ^ is_second {
            ans += 1;
        }
    }
    println!("{}", ans);
    Ok(())
}