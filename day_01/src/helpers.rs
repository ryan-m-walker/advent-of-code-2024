pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let split: Vec<_> = line.split("   ").collect();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    (left, right)
}

