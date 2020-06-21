use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    let mut ans: u64 = 0;
    for i in 1..n {
        ans += i;
    }
    println!("{}", ans);
}
