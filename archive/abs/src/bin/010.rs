use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    while !s.is_empty() {
        if s.ends_with("dreamer") {
            s.truncate(s.len() - 7);
        } else if s.ends_with("dream") {
            s.truncate(s.len() - 5);
        } else if s.ends_with("eraser") {
            s.truncate(s.len() - 6);
        } else if s.ends_with("erase") {
            s.truncate(s.len() - 5);
        } else {
            println!("NO");
            return
        }
    }

    println!("YES");
}
