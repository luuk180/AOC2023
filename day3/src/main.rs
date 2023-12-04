use std::fs::read_to_string;

fn main() {
    first();
    second();
}

fn first(){
    let input = read_to_string("./input.txt").unwrap();
    let binding = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    let y = binding.len();
    let x = binding[0].chars().count();
    for new_y in 0..y{
        let mut num = "".to_string();
        let mut around = "".to_string();
        for new_x in 0..x{
            let char = binding[new_y].chars().collect::<Vec<_>>()[new_x];
            if char.is_ascii_digit(){
                num.push(char);
                if new_y > 0{
                    if new_x > 0{around.push(get_char(&binding, new_x-1, new_y-1))}
                    around.push(get_char(&binding, new_x, new_y-1));
                }
                if new_y < y-1{
                    if new_x < x-1{
                        around.push(get_char(&binding, new_x+1, new_y+1))
                    }
                    around.push(get_char(&binding, new_x, new_y+1));
                }
                if new_x > 0{
                    if new_y < y-1{
                        around.push(get_char(&binding, new_x-1, new_y+1))
                    }
                    around.push(get_char(&binding, new_x-1, new_y));
                }
                if new_x < x-1{
                    if new_y > 0{
                        around.push(get_char(&binding, new_x+1, new_y-1))
                    }
                    around.push(get_char(&binding, new_x+1, new_y))
                }
            }else if !num.is_empty() && around.chars().any(|c| !c.is_ascii_digit() && c != '.')
            {sum += parse_int(num.to_string());
                around.clear();
            num = "".to_string()}
            else{
                around.clear();
                num = "".to_string()
            }
        }
        if !num.is_empty() && around.chars().any(|c| !c.is_ascii_digit() && c != '.')
        {sum += parse_int(num.to_string());
            around.clear()}
    }
    println!("The first solution is: {}", sum);
}

fn second(){
    println!("Kanker");
}

fn get_char(binding: &[&str], x: usize, y: usize) -> char{
    binding[y].chars().collect::<Vec<_>>()[x]
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
