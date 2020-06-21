use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: String
    }
    let len = s.len();
    for _ in (0..len - 1).rev() {
        s.pop();
        if s.len() % 2 == 1 {
            continue;
        } else {
            let half = s.len() / 2;
            let sub1 = &s[..half];
            let sub2 = &s[half..s.len()];
            if sub1 == sub2 {
                let ans = s.len();
                println!("{}", ans);
                return;
            }
        }
    }
}
