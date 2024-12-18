fn main() {
    let s1 = String::from("hello how are you");

    let s2 = first_word(&s1);


    println!("{s2}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}


