use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:u32,
        m:u32,
        a:[u32;m]
    }
    let mut ans = 0;
    for i in a {
        ans += i;
    }
    if ans < n {
        println!("{}", n - ans);
    } else {
        println!("{}", -1);
    }
}
