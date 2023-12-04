use std::fs::read_to_string;

fn main() {
    first();
}

fn first(){
    let input = read_to_string("./test_input.txt").unwrap();
    let binding = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    let y = binding.len();
    let x = binding[0].chars().count();
    for new_y in 0..y{
        let mut num = "".to_string();
        let mut valid = false;
        for new_x in 0..x{
            let char = binding[new_y].chars().collect::<Vec<_>>()[new_x];
            println!("{}: {}", new_x, new_y);
            if char.is_ascii_digit(){
                num.push(char);
                if new_x > 0 && new_y < y-1 && check_binding(&binding, new_x-1, new_y+1) {valid = true}
                if new_x < x-1 && new_y > 0 && check_binding(&binding, new_x+1, new_y-1) {valid = true}
                if new_x > 0 && new_y > 0 && check_binding(&binding, new_x-1, new_y-1) {valid = true}
                if new_x < x-1 && new_y < y-1 && check_binding(&binding, new_x+1, new_y+1) {valid = true; println!("{}", num)}
                if new_x > 0 && check_binding(&binding, new_x-1, new_y) {valid = true}
                if new_x < x-1 && check_binding(&binding, new_x+1, new_y) {valid = true}
                if new_y > 0 && check_binding(&binding, new_x, new_y-1) {valid = true}
                if new_y < y-1 && check_binding(&binding, new_x, new_y+1) {valid = true}
            }
            if valid && (new_x == x-1 || !char.is_ascii_digit()){
                println!("{}", num);
                sum += parse_int(num);
                num = str::to_string("");
                valid = false;
            }
            // if !valid && !char.is_ascii_digit(){num = "".to_string()}
        }
    }
    println!("The first solution is: {}", sum);
}

fn check_binding(binding: &Vec<&str>, x: usize, y: usize) -> bool{
    let char = binding[x].chars().collect::<Vec<_>>()[y];
    !char.is_ascii_digit() && char != '.'
}
fn parse_int(num: String) -> i32 {
    let mut new_num = 0;
    let parsed_num = num.parse::<i32>();
    if parsed_num.is_ok() {new_num = parsed_num.unwrap()}
    else{
        println!("{}", parsed_num.unwrap_err());
    }
    new_num
}
