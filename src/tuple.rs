pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mars", 38);

    println!("Tuple :");

    println!(
        "{} is from {} and he is {} years old",
        person.0, person.1, person.2
    );

    // Destructuring

    let (name, where_from, age) = person;

    println!(
        "Restructure: {} is from {} and he is {} years old",
        name, where_from, age
    );
}
