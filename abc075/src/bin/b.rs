use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut h:usize,
        mut w:usize,
        s:[String;h],
    }
    let mut vv = vec![vec![0; w]; h];
    h -= 1;
    w -= 1;

    for i in 0..h {
        let ch = s[i].chars().collect::<Vec<char>>();
        for k in 0..w {
            if ch[k] == '#' {
                if k != 0 && k != w {
                    vv[i][k - 1] += 1;
                    vv[i][k + 1] += 1;
                } else if k == 0 {
                    vv[i][k + 1] += 1;
                } else {
                    vv[i][k - 1] += 1;
                }

                if i != 0 && i != h {
                    vv[i - 1][k] += 1;
                    vv[i + 1][k] += 1;
                } else if i == 0 {
                    vv[i + 1][k] += 1;
                } else {
                    vv[i - 1][k] += 1;
                }

                if i != 0 && i != h && k != 0 && k != w {
                    vv[i - 1][k - 1] += 1;
                    vv[i - 1][k + 1] += 1;
                    vv[i + 1][k - 1] += 1;
                    vv[i + 1][k + 1] += 1;
                } else if i == 0 && k == 0 {
                    vv[i + 1][k + 1] += 1;
                } else if i == 0 && k != 0 && k != w {
                    vv[i + 1][k - 1] += 1;
                    vv[i + 1][k + 1] += 1;
                } else if i == 0 && k == w {
                    vv[i + 1][k - 1] += 1;
                } else if i != 0 && k == w && i != h {
                    vv[i - 1][k - 1] += 1;
                    vv[i + 1][k - 1] += 1;
                } else if i != 0 && k == 0 && i != h {
                    vv[i - 1][k + 1] += 1;
                    vv[i + 1][k + 1] += 1;
                } else if i == h && k == 0 {
                    vv[i - 1][k + 1] += 1;
                } else if i == h && k != 0 && k != w {
                    vv[i - 1][k + 1] += 1;
                    vv[i - 1][k - 1] += 1;
                } else {
                    vv[i - 1][k - 1] += 1;
                }
            }
        }
    }
    println!("{:?}", vv);
}
