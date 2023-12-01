use std::fs::read_to_string;
use std::ops::{Add};

const ONES: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"];
fn main() {
    first();
    second();
}

fn first(){
    println!("This is the first solution!");
    let mut sum = 0;
    for line in read_to_string("./input.txt").unwrap().lines(){
        let mut num = "".to_owned();
        for char in line.chars(){
            if char.is_digit(10) {num.push(char)}
        }
        let first = first_last(num);
        sum += parse_int(first);

    }
    println!("Solution one total: {}", sum);
}

fn second(){
    println!("This is the second solution!");
    let mut sum = 0;
    for line in read_to_string("./input.txt").unwrap().lines(){
        let mut new_line = line.to_owned();
        let mut count = "".to_owned();
        for digit in 0..ONES.len(){
            new_line = new_line.replace(ONES[digit], &*(ONES[digit].chars().nth(0).unwrap().to_string() + &*digit.to_string() + &*ONES[digit].chars().nth_back(0).unwrap().to_string()));
        }
        for char in new_line.chars(){
            if char.is_digit(10){
                count.push(char);
            }
        }
        sum += parse_int(first_last(count));
    }
    println!("Soltuion two total: {}", sum);
}

pub fn first_last(line: String) -> String {
    return line.chars().nth(0).unwrap().to_string().add(&*line.chars().nth_back(0).unwrap().to_string());
}

pub fn parse_int(num: String) -> i32 {
    let mut new_num = 0;
    let parsed_num = num.parse::<i32>();
    if parsed_num.is_ok() {new_num = parsed_num.unwrap()}
    else{
        println!("{}", parsed_num.unwrap_err());
    }
    return new_num;
}