use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.chars().filter(|&c| c == '1').count();

    println!("{}", ans);
}