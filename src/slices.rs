pub fn run() {
    let messages = String::from("Greetings from Earth!");

    println!("message is {}", messages);

    let last_word = &messages[15..];

    println!("last_word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let inner_planets: &[i32] = &planets[..4];

    println!("inner_planets is {:?}", inner_planets);

    let first_woord: &str = get_first_word(&messages); // cannot send slide as parameter expecting String

    println!("first_woord is {:?}", first_woord);

    let first_word: &str = get_word(&messages[10..]);

    println!("first_woord is {:?}", first_word);
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }

    return &s; // No space found. input is a single word
}

fn get_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }

    return &s;
}
