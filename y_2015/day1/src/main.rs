fn read_input() -> String {
    std::fs::read_to_string("input_day1.txt").expect("missing file")
}

fn main() {
    let mut floor = 0;
    for action in read_input().chars() {
        if action == '(' {
            floor += 1
        }
        if action == ')' {
            floor -= 1
        }
    }
    println!("sol 1: {floor}");

    let mut result = 0;
    let mut floor = 0;
    for action in read_input().chars() {
        result +=1;
        if action == '(' {
            floor += 1
        }
        if action == ')' {
            floor -= 1
        }
        if floor == -1 {
            break
        }
    }
    println!("sol 2: {result}");
}
