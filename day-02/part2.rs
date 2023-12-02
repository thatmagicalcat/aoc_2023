fn main() {
    println!(
        "ansewr: {answer}",
        answer = solve(include_str!("../../input.txt"))
    );
}

fn solve(input: &str) -> usize {
    input.lines().filter_map(|line| parse_game(line)).sum()
}

fn parse_game(line: &str) -> Option<usize> {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        panic!()
    }

    for part in parts[2..].chunks(2) {
        let &[number, color] = part else {
            panic!("invalid part: {part:?}");
        };

        let color = color.trim_matches(|c| c == ';' || c == ',');
        let number: usize = number.parse().unwrap();

        match color {
            "red" => max_red = max_red.max(number),
            "green" => max_green = max_green.max(number),
            "blue" => max_blue = max_blue.max(number),
            _ => panic!()
        };
    }

    Some(max_red * max_green * max_blue)
}
