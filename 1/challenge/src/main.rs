use std::env;
use std::fs::read_to_string;

fn main(){
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = read_lines(&file_path);
    let mut vect_tuple = to_tuple(&file);

    let mut result: i32 = 0;


    while !vect_tuple.1.is_empty() {

        let min_left: &i32 = vect_tuple.0.iter().min()
            .expect("Error catching num.");
        let min_right: &i32 = vect_tuple.1.iter().min()
            .expect("Error catching num.");

        let diff: i32 = match min_left > min_right {
            true => min_left - min_right,
            false => min_right - min_left,
        };

        result += diff;

        let index_left: usize = vect_tuple.0.iter().position(|num| num == min_left)
            .expect("Error indexing tuple");

        let index_right: usize = vect_tuple.1.iter().position(|num| num == min_right)
            .expect("Error indexing tuple");

        println!("{index_left}:{min_left}");
        println!("{index_right}:{min_right}");
        vect_tuple.0.remove(index_left);
        vect_tuple.1.remove(index_right);
    }
    println!("{result}");
}

fn read_lines(file_name: &String) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(file_name).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn to_tuple(input: &Vec<String>) -> (Vec<i32>, Vec<i32>) {

    let mut first_vect: Vec<i32> = Vec::new(); 
    let mut second_vect: Vec<i32> = Vec::new(); 

    for line in input {
        let first_num = line[..5].parse::<i32>()
            .expect("Error parsing");
        let second_num = line[8..].parse::<i32>()
            .expect("Error parsing");

        first_vect.push(first_num);
        second_vect.push(second_num);
    }
    let output_tuple: (Vec<i32>, Vec<i32>) = (first_vect, second_vect);

    return output_tuple;
}
