use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut min_count = i64::MAX;

    for mut x in a {
        let mut count = 0;
        while x % 2 == 0 {
            x /= 2;
            count += 1;
        }

        min_count = min_count.min(count); 
    }

    println!("{}", min_count);
}
