use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [i64; n]
    }

    x.sort_by(|x, y| y.cmp(x));

    let mut a = 0;
    let mut b = 0;

    for (i ,v) in x.into_iter().enumerate() {
        if i % 2 == 0 {
            a += v;
        } else {
            b += v;
        }
    }

    let ans = a - b;

    println!("{}", ans);
}
