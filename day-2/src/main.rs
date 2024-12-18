use std::env;
use std::fs;
use std::io::Error;

/*
    Nosed reactor safety systems can only tolerate levels that are
    either gradually increasing or gradually decreasing.
    So, a report only counts as safe if both of the following are true:
        * The levels are either all increasing or all decreasing.
        * Any two adjacent levels differ by at least one and at most three.

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

*/

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<_>>();
    let file_name: &str = &args[1];

    let reports = read_file(file_name).unwrap();
    let mut safe_reports = 0;

    for report in reports {
        if check_report(&report).unwrap() {safe_reports += 1}
    }

    println!("number of safe reports: {safe_reports}");

    Ok(())
}

fn check_report(report: &Vec<i32>) -> Result<bool, Error> {
    match check_adjacent(&report).unwrap() && check_monotonic(&report).unwrap() {
        true => return Ok(true),
        false => return Ok(false),

    };
}

fn check_adjacent(report: &Vec<i32>) -> Result<bool, Error> {
    for i in 0..report.len() - 1 {
        let result = (report[i] - report[i + 1]).abs();
        if result == 0 || result > 3 {
            return Ok(false);
        }
    }
    Ok(true)
}

fn check_monotonic(report: &Vec<i32>) -> Result<bool, Error> {
    let mut increasing: bool = true;
    let mut decreasing: bool = true;

    for i in 0..report.len() - 1 {
        if report[i] < report[i + 1] {
            decreasing = false;
        }
        if report[i] > report[i + 1] {
            increasing = false; 
        }
    }

    Ok(increasing || decreasing)
}

fn read_file(f: &str) -> Result<Vec<Vec<i32>>, Error> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in fs::read_to_string(f).unwrap().lines() {
        let nums = line
            .split(" ")
            .map(|num| -> i32 { num.parse().unwrap() })
            .collect::<Vec<_>>();
        reports.push(nums);
    }
    Ok(reports)
}
