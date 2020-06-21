fn main() {
    let s = "Hello, world!";
    search_word(s);
}

fn search_word(s: &str) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
