use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        queries: [(usize, usize); q],
    }

    // 累積和
    let mut sum = vec![0; n + 1];

    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    // 質問に答える
    for (l, r) in queries {
        println!("{}", sum[r] - sum[l - 1]);
    }
}