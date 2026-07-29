use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let words = vec![
        "maerd",
        "remaerd",
        "esare",
        "resare",
    ];

    let mut s: String = s.chars().rev().collect();

    while s.len() > 0 {
        let mut deleted = false;

        for word in &words {
            if s.starts_with(word) {
                s = s[word.len()..].to_string();
                deleted = true;
                break;
            }
        }

        if !deleted {
            println!("NO");
            return;
        }
    }

    println!("YES");
}