// use std::env;

pub fn run() {
    // let args: Vec<String> = std::env::args().collect();
    // let command = args[1].clone();

    let mut args = std::env::args().skip(1);
    let command = args.next().expect("Command not entered!");
    let name = "Alan";
    let status = "100%";

    // println!("Args: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you ?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Command '{}' is invalid", command);
    }
}
