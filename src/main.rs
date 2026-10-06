use std::io;
fn main() {
    let mut input = String::new();
    let mut buff = String::new();

    while input.len() < 3 {
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read line");
        match buff.as_str() {
            " " | "\t" | "\n" => continue,
            _ => input = input + " " + buff.as_str().trim(),
        };

        buff = "".to_string();
    }

    let parts: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", parts[0] + parts[1]);
}
