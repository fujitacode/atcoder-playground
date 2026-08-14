use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i32; w]; h],
        q: usize,
        queries: [(usize, usize, usize, usize); q],
    }

    let mut prefix = vec![vec![0; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            prefix[i + 1][j + 1] =
                prefix[i][j + 1]
                + prefix[i + 1][j]
                - prefix[i][j]
                + x[i][j];
        }
    }

    for (a, b, c, d) in queries {
        let ans =
            prefix[c][d]
            - prefix[a - 1][d]
            - prefix[c][b - 1]
            + prefix[a - 1][b - 1];

        println!("{}", ans);
    }
}