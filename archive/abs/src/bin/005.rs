use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        x: i64
    }

    let mut count = 0;

    for i in 0..= a {
        for j in 0..= b {
            for k in 0..= c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                } 
            }
        }
    }

    println!("{}", count);
}
