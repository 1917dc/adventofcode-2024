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
    println!("the sum is {p2}");

    Ok(())
}

fn p1(f: &str) -> Result<u32, Error> {
    let re = Regex::new(r"mul\((?<num1>[0-9]{1,3}),(?<num2>([0-9]{1,3}))\)").unwrap();
    let mut result: u32 = 0;
    for caps in re.captures_iter(&fs::read_to_string(f)?) {
        if let (Some(num1), Some(num2)) = (caps.name("num1"), caps.name("num2")) {
            let num1 = num1.as_str().parse::<u32>().unwrap();
            let num2 = num2.as_str().parse::<u32>().unwrap();
            result += num1 * num2;
        }
    }
    Ok(result)
}

fn p2(f: &str) -> Result<u32, Error> {
    let re = Regex::new(
        r"(?<cmd_do>do\(\))|(?<cmd_dont>don't\(\))|mul\((?<num1>[0-9]{1,3}),(?<num2>[0-9]{1,3})\)",
    )
    .unwrap();
    let content = fs::read_to_string(f)?;
    let mut enabled = true;
    let mut result: u32 = 0;
    for cap in re.captures_iter(&content) {
        if cap.name("cmd_do").is_some() {
            enabled = true;
        } else if cap.name("cmd_dont").is_some() {
            enabled = false;
        } else if let (Some(num1), Some(num2)) = (cap.name("num1"), cap.name("num2")) {
            if enabled {
                let num1 = num1.as_str().parse::<u32>().unwrap();
                let num2 = num2.as_str().parse::<u32>().unwrap();
                result += num1 * num2;
            }
        }
    }
    Ok(result)
}
