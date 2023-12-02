use std::cmp::max;
use std::fs::read_to_string;

fn main() {
    first();
    second();
}

fn first(){
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sum = 0;
    for line in read_to_string("./input.txt").unwrap().lines(){
        let left_right = line.split(":").collect::<Vec<_>>();
        let id = parse_int(left_right[0].trim_start_matches("Game "));
        let mut valid = true;

        for part in left_right[1].split(|c| c == ',' || c == ';' ).collect::<Vec<_>>(){
            let split_part = part.trim_start().split(" ").collect::<Vec<_>>();
            match split_part[1].trim() {
                "red" => if parse_int(split_part[0].trim()) > max_red {valid = false},
                "green" => if parse_int(split_part[0].trim()) > max_green {valid = false},
                "blue" => if parse_int(split_part[0].trim()) > max_blue {valid = false},
                _ => {}
            }
        }

        if valid {
            sum += id
        }
    }
    println!("The solution to problem one is: {}", sum);
}

fn second(){
    let mut sum = 0;
    for line in read_to_string("./input.txt").unwrap().lines(){
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for part in line.split(": ").collect::<Vec<_>>()[1].split(|c| c == ',' || c == ';').collect::<Vec<_>>(){
            let split_game = part.trim_start().split(" ").collect::<Vec<_>>();
            match split_game[1] {
                "red" => if parse_int(split_game[0]) > max_red {max_red = parse_int(split_game[0])},
                "green" => if parse_int(split_game[0]) > max_green {max_green = parse_int(split_game[0])},
                "blue" => if parse_int(split_game[0]) > max_blue {max_blue = parse_int(split_game[0])},
                _ => {}
            }
        }
        sum += max(max_red, 1) * max(max_green, 1) * max(max_blue, 1);
    }

    println!("The solution to problem two is: {}", sum);
}

pub fn parse_int(num: &str) -> i32 {
    let mut new_num = 0;
    let parsed_num = num.parse::<i32>();
    if parsed_num.is_ok() {new_num = parsed_num.unwrap()}
    else{
        println!("{}", parsed_num.unwrap_err());
    }
    return new_num;
}