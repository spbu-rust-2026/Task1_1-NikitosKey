use std::io;
fn main() {
    let lines = io::stdin().lines();
    let mut sum: i128 = 0;
    let mut counter: u8 = 0;

    for line in lines {
        for token in line.unwrap().split_whitespace() {
            match token {
                " " | "\t" | "\n" => continue,
                val if counter < 2 => {
                    sum += val.parse::<i64>().unwrap() as i128;
                    counter += 1;
                }
                _ => break,
            }
        }
        match counter {
           val if val == 2 as u8 => break,
           _ => continue
        }
    }
    println!("{}", sum);
}
