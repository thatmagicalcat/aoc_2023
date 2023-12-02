fn main() {
    let input = include_str!("input.txt");
    let answer: u32 = input
        .lines()
        .map(|l| {
            let mut i = l.chars().filter(|i| i.is_ascii_digit());
            let a = i.next().unwrap();
            let b = i.last().unwrap_or(a);

            a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()
        })
        .sum();

    println!("answer: {answer}");
}
