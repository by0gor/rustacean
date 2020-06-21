use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32
    }
    let mut v = vec![a, b, c];
    v.sort();
    let ans = v[0] + v[1];
    println!("{}", ans);
}
