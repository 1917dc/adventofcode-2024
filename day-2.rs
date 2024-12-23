use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<_>>();
    let file_name: &str = &args[1];

    let reports = read_file(file_name)?;
    let mut valid_p1 = 0;
    let mut valid_p2 = 0;

    for report in reports {
        let p1 = is_report_safe(&report, false)?;
        let p2 = is_report_safe(&report, true)?;
        if p1 {
            valid_p1 += 1
        }
        if p2 {
            valid_p2 += 1
        }
        println!("report: {report:?} | valid: {p2}");
    }
    println!("number of valid reports on stage 1: {valid_p1}");
    println!("number of valid reports on stage 2: {valid_p2}");

    Ok(())
}

fn is_safe(report: &Vec<i32>) -> Result<bool, Error> {
    let is_increasing = report.windows(2).all(|num| num[1] > num[0]);
    let is_decreasing = report.windows(2).all(|num| num[0] > num[1]);

    if !is_increasing && !is_decreasing {
        return Ok(false);
    }

    Ok(report.windows(2).all(|num| (num[1] - num[0]).abs() <= 3))
}

fn is_report_safe(report: &Vec<i32>, use_dampener: bool) -> Result<bool, Error> {
    if is_safe(report)? {
        return Ok(true);
    }

    if use_dampener {
        for i in 0..report.len() {
            let mut report_copy = report.clone();
            report_copy.remove(i);
            if is_safe(&report_copy)? {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn read_file(f: &str) -> Result<Vec<Vec<i32>>, Error> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in fs::read_to_string(f)?.lines() {
        let nums = line
            .split(" ")
            .map(|num| -> i32 { num.parse().unwrap() })
            .collect::<Vec<_>>();
        reports.push(nums);
    }
    Ok(reports)
}
