pub fn run() {
    let mut hello = String::from("Hello ");

    println!("String :");

    //get Length

    println!("Length of string : {}", hello.len());

    // Push a character

    hello.push('W');

    // push a string

    hello.push_str("orld!");

    println!("{}", hello);

    hello.push_str(" Welcome!");

    println!("{}", hello);

    // print capacity in bytes

    println!("Capacity : {}", hello.capacity());
    println!("Contain World : {}", hello.contains("World"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing

    assert_eq!(2, s.len());
}
