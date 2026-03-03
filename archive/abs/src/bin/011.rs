use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut prev_t: i64 = 0;
    let mut prev_x: i64 = 0;
    let mut prev_y: i64 = 0;

    for _ in 0..n {
        input! {
            t: i64,
            x: i64,
            y: i64,
        }

        let dt = t - prev_t;
        let dist = (x - prev_x).abs() + (y - prev_y).abs();

        if dt < dist || (dt - dist) % 2 != 0 {
            println!("No");
            return;
        }

        prev_t = t;
        prev_x = x;
        prev_y = y;
    }

    println!("Yes");
}