use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32
    }
    if a != b && a != c {
        println!("{}", a);
    } else if b != a && b != c {
        println!("{}", b);
    } else {
        println!("{}", c);
    }
}
