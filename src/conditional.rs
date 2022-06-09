pub fn run() {

    let age = 18;

    println!("Conditional Statement : ");

    if age >= 21 {
        println!("Age is >= 21");
    } else if age < 21 {
        println!("Age is < 21");
    };

    if age >= 18 && age < 21 {
        println!("Age is almost there");
    }

    // or

    let is_age = if age > 21 { true } else { false};

    if is_age { println!("Age is >= 21")} else { println!("Age is < 21")};


}