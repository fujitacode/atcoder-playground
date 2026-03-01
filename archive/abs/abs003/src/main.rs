use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.chars().filter(|&x| x == '1').count();

    println!("{}", ans);
}
