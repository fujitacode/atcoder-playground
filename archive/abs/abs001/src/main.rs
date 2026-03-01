use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        s: String
    }

    let num = a + b + c;
    let str = s;

    println!("{} {}", num, str);
}
