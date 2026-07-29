use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }

    for i in a {
        if i == x {
            println!("Yes");
            return;
        }
    }

    println!("No");
}