use std::io::Write;
use std::process::Command;

const TEMPLATE_DIR: &str = "template";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("Name not specified");
    }

    let name = &args[1];

    let success = Command::new("cp")
        .arg("-r")
        .arg(TEMPLATE_DIR)
        .arg(name)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    if success {
        println!("Created a new directory");
    } else {
        panic!("Cannot create a new directory");
    }

    let content = format!(
        r#"
[[bin]]
name = "{name}-p1"
path = "{name}/part1.rs"

[[bin]]
name = "{name}-p2"
path = "{name}/part2.rs""#
    );

    let cargo_file_contents = std::fs::read_to_string("Cargo.toml").unwrap();
    let mut cargo_file =
        std::fs::File::create("Cargo.toml").unwrap();

    cargo_file.write_all(format!("{cargo_file_contents}{content}").as_bytes()).unwrap();
}
