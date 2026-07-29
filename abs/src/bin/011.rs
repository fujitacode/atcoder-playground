use proconio::input;

fn main() {
    input! {
        n: usize,
        z: [(i32, i32, i32); n],
    }

    let mut prev_t = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;

    for (t, x, y) in z {
        let dt = t - prev_t;
        let dist = (x - prev_x).abs() + (y - prev_y).abs();

        if dist > dt || (dt - dist) % 2 != 0 {
            println!("No");
            return;
        }

        prev_t = t;
        prev_x = x;
        prev_y = y;
    }

    println!("Yes");
}