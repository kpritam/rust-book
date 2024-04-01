fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn run() {
    let input = "Hello World!";
    let fw = first_word(input);

    println!("First word of {} is {}", input, fw)
}