use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n-1]
    }
    let mut num = vec![0; n];

    for l in a {
        let m: usize = l - 1;
        num[m] += 1;
    }
    for k in num {
        println!("{:?}", k);
    }
}
