use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        d: usize,
        queries: [(usize, usize); d],
    }

    let mut left = vec![0; n + 1];
    let mut right = vec![0; n + 2];

    for i in 1..=n {
        left[i] = left[i - 1].max(a[i - 1]);
    }

    for i in (1..=n).rev() {
        right[i] = right[i + 1].max(a[i - 1]);
    }

    for (l, r) in queries {
        let ans = left[l - 1].max(right[r + 1]);
        println!("{}", ans);
    }
}