fn read_input() -> String {
    std::fs::read_to_string("input_day1.txt").expect("missing file")
}

fn main() {

    let mut max = 0;
    for group in read_input().split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().expect("error");
            sum += value;
        }
        if sum > max {
            max = sum;
        }
    }
    println!("sol 1: {max}");

    let mut second_max = 0;
    for group in read_input().split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().expect("error");
            sum += value;
        }
        if sum > second_max && sum < max {
            second_max = sum;
        }
    }

    let mut third_max = 0;
    for group in read_input().split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().expect("error");
            sum += value;
        }
        if sum > third_max && sum < second_max {
            third_max = sum;
        }
    }

    let result = max + second_max + third_max;
    println!("sol 2: {result}");
}
