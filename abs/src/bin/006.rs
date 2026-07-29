use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut total_sum = 0;

    for i in 1..=n {
        let mut x = i;
        let mut sum = 0;
 
        while x > 0 {
            sum += x % 10;
            x /= 10;
        }

        if a <= sum && sum <= b {
            total_sum += x;
        }
    }

    println!("{}", total_sum);
}