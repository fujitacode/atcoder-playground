use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64
    }

    let mut total = 0;

    for i in 1..=n {
        let mut x = i;
        let mut sum = 0;

        while x > 0 {
            sum += x % 10;
            x /= 10;
        }

        if a <= sum && sum <= b {
            total += i;
        }
    }

    println!("{}", total);
}