use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let len: usize = s.len() + 1;
    let ch = s.chars().collect::<Vec<char>>();
    let mut left = vec![0; len];
    let mut count_left: u64 = 0;
    let mut count_right: u64 = 0;
    for i in 1..len {
        if ch[i - 1] == '<' {
            count_left += 1;
        } else {
            count_left = 0;
        }
        left[i] = count_left;
    }
    for i in (0..len - 1).rev() {
        if ch[i] == '>' {
            count_right += 1;
        } else {
            count_right = 0;
        }
        left[i] = std::cmp::max(left[i], count_right);
    }
    let ans: u64 = left.into_iter().sum();
    println!("{}", ans);
}
