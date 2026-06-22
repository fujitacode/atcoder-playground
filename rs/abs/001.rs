use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input.split_whitespace()
                           .map(|s| s.parse().unwrap())
                           .collect();
    let (a, b) = (v[0], v[1]);

    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}