use std::cmp::{min};
use std::fs::read_to_string;
use std::str::Chars;

fn main() {
    first();
    second();
}

fn first(){
    let mut sum = 0;
    for line in read_to_string("./input.txt").unwrap().lines(){
        let parts = line.split(':').collect::<Vec<_>>()[1].split('|').collect::<Vec<_>>();
        let winning = parse_nums(parts[0].chars());
        let actual = parse_nums(parts[1].chars());
        let mut curr_num = 0;
        for num in actual{
            if winning.contains(&num) {
                if curr_num > 1 { curr_num *= 2 } else {
                    curr_num += 1;
                }
            }
        }
        sum += curr_num;
    }
    println!("The first solution is: {}", sum);
}

fn second(){
    let mut worth: Vec<usize> = vec![];
    for line in read_to_string("./input.txt").unwrap().lines(){
        let parts = line.split(':').collect::<Vec<_>>()[1].split('|').collect::<Vec<_>>();
        let winning = parse_nums(parts[0].chars());
        let actual = parse_nums(parts[1].chars());
        let mut curr_num = 0;
        for num in actual{
            if winning.contains(&num) {
                curr_num += 1;
            }
        }
        worth.push(curr_num);
    }
    let mut quant: Vec<usize> = vec![0;worth.len()];
    for thing in 0..worth.len(){
        quant[thing] += 1;
        for _ in 0..quant[thing]{
        for new_thing in thing+1..min(thing+worth[thing]+1, worth.len()) {
            quant[new_thing] += 1;
        }}
    }
    println!("The second solution is: {}", quant.iter().sum::<usize>());
}

fn parse_nums(chars: Chars) -> Vec<usize>{
    let mut nums: Vec<usize> = vec![];
    let mut real_num = "".to_string();
    for char in chars {
        if char.is_ascii_digit() {real_num.push(char);}
        else if !real_num.is_empty() {nums.push(parse_int(real_num)); real_num = "".to_string()}
    }
    if !real_num.is_empty() {nums.push(parse_int(real_num))}
    nums
}

fn parse_int(num: String) -> usize {
    let mut new_num = 0;
    let parsed_num = num.parse::<usize>();
    if parsed_num.is_ok() {new_num = parsed_num.unwrap()}
    else{
        println!("{}", parsed_num.unwrap_err());
    }
    new_num
}
