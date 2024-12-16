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

    for report in reports {
        check_report(&report);
    }

    Ok(())
}

fn check_report(report: &Vec<i32>) -> Result<bool, Error> {
    if report[0] <= report[1]
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