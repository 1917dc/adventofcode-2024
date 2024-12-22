use regex::Regex;
use std::env;
use std::fs;
use std::io::{Error, Read};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    println!("File: {}", file);
    let p1 = p1(file).unwrap();
    println!("the sum is {p1}");

    let p2 = p2(file).unwrap();

    Ok(())
}

fn p1(f: &str) -> Result<u32, Error> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut nums: Vec<[u32; 2]> = Vec::new();

    for (_, [a, b]) in re
        .captures_iter(&fs::read_to_string(f)?)
        .map(|c| c.extract())
    {
        nums.push([a.parse().unwrap(), b.parse().unwrap()]);
    }

    let mut res: Vec<u32> = Vec::new();
    for arr in nums {
        res.push(arr[0] * arr[1]);
    }
    Ok(res.iter().sum::<u32>())
}

fn p2(f: &str) -> Result<(), Error> {
    


    Ok(())
}
