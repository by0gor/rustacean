use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut b: Vec<usize> = Vec::new();

    if n % 2 == 0 {
        for i in (1..n + 1).rev() {
            if i % 2 == 0 {
                b.push(a[i - 1]);
            }
        }
        for i in 1..n + 1 {
            if i % 2 == 1 {
                b.push(a[i - 1]);
            }
        }

        for item in b {
            println!("{} ", item);
        }
    } else {
        for i in (1..n + 1).rev() {
            if i % 2 == 1 {
                b.push(a[i - 1]);
            }
        }
        for i in 1..n + 1 {
            if i % 2 == 0 {
                b.push(a[i - 1]);
            }
        }
        for item in b {
            println!("{} ", item);
        }
    }
}
