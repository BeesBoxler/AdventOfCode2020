use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;
use std::str::FromStr;

const TARGET:i32 = 2020;

fn main() {
    let input = File::open("day01.txt").expect(
        "No Such File"
    );
    let reader = BufReader::new(input);
    let mut values = reader.lines()
        .map(|l| i32::from_str(&(l.expect("No Line"))).expect("Not a number"))
        .filter(|&l| l < TARGET)
        .collect::<Vec<i32>>();

    values.sort();

    let val_len = values.len();
    let mut found = false;

    println!("Starting calculations... bleep bloop...");

    for i in 0..val_len {
        let i_val = values[val_len-1 - i];
        for j in 0..val_len{
            if i_val + values[j] > TARGET {break}
            else if i_val + values[j] == TARGET {
                found = true; 
                println!("{} + {} = {}", i_val, values[j] , i_val + values[j]);
                println!("{} * {} = {}", i_val, values[j] , i_val * values[j]);
            }
        }
        if found {break};
    };
found = false;
    for i in 0..val_len {
        for j in 0..val_len{
            for k in 0..val_len{
                if values[i] + values[j] + values[k]> TARGET {break}
                else if values[i] + values[j] + values[k] == TARGET {
                    found = true; 
                    println!("{} + {} + {} = {}", values[i], values[j] , values[k], values[i] + values[j] + values[k]);
                    println!("{} * {} * {} = {}", values[i], values[j] , values[k], values[i] * values[j] * values[k]);
                }
            }
            if found {break};
        }
        if found {break};
    };
}
