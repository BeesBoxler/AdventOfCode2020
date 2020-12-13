use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::str::FromStr;



// https://adventofcode.com/2020/day/2

fn main() {
    let input = File::open("day02.txt").expect("File Doesn't Exist");
    let reader = BufReader::new(input);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();


    let line_re = Regex::new(r"(\d+-\d+) (\w): (\w*)").unwrap();

    let mut correct_passwords = 0;
    let mut correct_passwords_pt2 = 0;

    for l in lines {
        let m = line_re.captures(&l).unwrap();
        let count = &m[1];
        let target = &m[2];
        let to_search = &m[3];

        let mut found = false;

        let (min, max) = get_target_count(count);
        let found_count = get_count(to_search, target);
        if found_count >= min && found_count <= max{
            correct_passwords = correct_passwords + 1;
            found = true;
        }

        let mut min_xor_max = 0;

        let character_at_min_pos = &to_search[min-1..min];
        let character_at_max_pos = &to_search[max-1..max];
        if character_at_min_pos == target {min_xor_max = min_xor_max + 1;}
        if character_at_max_pos == target {min_xor_max = min_xor_max + 1;}


        

        print!("Character at position {} is {}, ", min, character_at_min_pos);
        print!("Character at position {} is {}...", max, character_at_max_pos);
        print!("xor value is {}", min_xor_max);

        if min_xor_max == 1 {
            correct_passwords_pt2 = correct_passwords_pt2 +1;
        }



        // print!("Finding {} in {}...", target, to_search);
        // print!(" Found {:?}... ", found_count);
        // print!("Expecting between {} and {}...", min, max);
        // if found {
        //     print!("✔");
        // } else {
        //     print!("❌");
        // }

        println!();

    }

    println!("Correct Passwords: {}", correct_passwords);
    println!("Correct Passwords pt2: {}", correct_passwords_pt2);
}

fn get_count(to_search: &str, target: &str) -> usize {
    let re = Regex::new(format!("({})", target).as_str()).unwrap();
    re.find_iter(to_search).count()
}

fn get_target_count(target: &str) -> (usize, usize) {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let cap = re.captures(target).unwrap();
    let a = str::parse::<usize>(&cap[1]).unwrap();
    let b = str::parse::<usize>(&cap[2]).unwrap();
    let min: usize;
    let max: usize;

    if a <= b{
        min = a;
        max = b;
    } else {
        max = a;
        min = b;
    }

    (min, max)
}

