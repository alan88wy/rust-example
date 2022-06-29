pub fn run() {
    greeting("Hello", "James");

    let get_sum = add(5, 10);

    println!("The sum of {}", get_sum);

    // Closure

    let n3 = 10;

    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("The sum of add_sum : {}", add_sum(3, 3));

    let (x, sq) = square(13);

    println!("Square of {} is {}", x, sq);

    let str = String::from("Hello");

    pass_by_value(str);

    // println!("Value of str is {}", str); // this will failed unless we do print_string(str.clone())
    //                                          or get function to return the value back to the variable
    //                                          eg let str = print_string(str). print_string will be define as
    //                                          fn print_string(s: String) -> String { return (s);}

    let mut str2 = String::from("Hello");

    pass_by_reference(&mut str2);

    println!("Final value of str2 is {}", str2);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(m: i32, n: i32) -> i32 {
    m + n
}

fn square(x: i32) -> (i32, i32) {
    return (x, x * x);
}

fn pass_by_value(s: String) {
    println!("Value of s is {}", s);
}

// pass by reference ? or Borrowing Reference

fn pass_by_reference(s: &mut String) {
    s.push_str(" World");
    println!("Value of borrowed str is {}", s);
}
