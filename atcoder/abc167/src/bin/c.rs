use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: u64
    }
    let mut c = vec![];
    let mut a = vec![];

    for _ in 0..n {
        input! {
            cv: u64,
            av: [u64; m]
        }
        c.push(cv);
        a.push(av);
    }
    let inf: u64 = 1 << 63 - 1;
    let mut ans = inf;

    for bits in 0..(1 << n) {
        let mut score = vec![0; m];
        let mut cost = 0;

        for i in 0..n {
            if bits & (1 << i) != 0 {
                cost += c[i];
                for (k, &item) in a[i].iter().enumerate() {
                    score[k] += item;
                }
            }
        }

        if score.iter().all(|&point| point >= x) {
            if cost < ans {
                ans = cost;
            }
        }
    }
    if ans > inf / 10 {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
