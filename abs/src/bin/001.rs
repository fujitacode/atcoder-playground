use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        t: i32,
    }

    let sum = a + b + c;

    println!("{} {}", sum, t);
}