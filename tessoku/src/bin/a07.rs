use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        plans: [(usize, usize); n],
    }

    let mut diff = vec![0; d + 2];

    for (l, r) in plans {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut attendance = 0;

    for day in 1..=d {
        attendance += diff[day];
        println!("{}", attendance);
    }
}