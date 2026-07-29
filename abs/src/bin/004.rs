use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut count = i32::MAX;

    for mut x in a {
        let mut c = 0;

        while x % 2 == 0 {
            x /= 2;
            c += 1;
        } 

        count = count.min(c);
    }

    println!("{}", count);
}