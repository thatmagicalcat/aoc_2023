fn main() {
    println!(
        "ansewr: {answer}",
        answer = solve(include_str!("input.txt"))
    );
}

fn solve(input: &str) -> usize {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    input
        .lines()
        .filter_map(|line| parse_game(line, red_cubes, green_cubes, blue_cubes))
        .sum()
}

fn parse_game(line: &str, red: usize, green: usize, blue: usize) -> Option<usize> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        panic!()
    }

    // Parse game ID
    let id = parts[1].trim_end_matches(':').parse::<usize>().ok()?;

    for part in parts[2..].chunks(2) {
        let &[number, color] = part else {
            panic!("invalid part: {part:?}");
        };

        let color = color.trim_matches(|c| c == ';' || c == ',');

        let number = number.parse().unwrap();

        match color {
            "red" if red >= number => {}
            "green" if green >= number => {}
            "blue" if blue >= number => {}
            _ => return None,
        };
    }

    Some(id)
}
