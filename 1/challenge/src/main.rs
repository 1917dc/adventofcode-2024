use std::fs;
use std::io;
use std::env;

fn main() -> std::io::Result<()> {
    let _args: Vec<String> = env::args().collect();
    let file_path = &_args[1];

    let result = read_file(&file_path).unwrap();

    let similarity = find_similarity(&result).unwrap();
    let distance = find_distance(result).unwrap();

    println!("The similarity is: {similarity}");
    println!("The distance is: {distance}");
    Ok(())
}

fn find_distance(mut arr: [Vec<i32>; 2]) -> Result<i32, io::Error> {
    let mut distance: i32 = 0;
    while !arr[1].is_empty() {
        let min_left: &i32 = arr[0].iter().min().unwrap();
        let min_right: &i32 = arr[1].iter().min().unwrap();

        let diff: i32 = match min_left > min_right {
            true => min_left - min_right,
            false => min_right - min_left,
        };    
        distance += diff;

        let index_left: usize = arr[0].iter().position(|num| num == min_left).unwrap();
        let index_right: usize = arr[1].iter().position(|num| num == min_right).unwrap();

        arr[0].remove(index_left);
        arr[1].remove(index_right);
    }

    Ok(distance)
}

fn read_file(filename: &str) -> Result<[Vec<i32>; 2], io::Error> {
    let mut result: [Vec<i32>; 2] = [Vec::new(), Vec::new()];
    for line in fs::read_to_string(filename).unwrap().lines() {
        result[0].push(line[..5].parse::<i32>().unwrap());
        result[1].push(line[8..].parse::<i32>().unwrap());
    }
    Ok(result)
}

fn find_similarity(arr: &[Vec<i32>; 2]) -> Result<i32, io::Error> {
    let mut similarity: i32 = 0;
    for i in &arr[0] {
        let mut count = 0;
        for j in &arr[1] {
            if i == j {
                count+=1;
            }
        }
        similarity += i * count;
    }
    Ok(similarity)
}
