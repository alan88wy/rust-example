// Variables hold primitive data or reference to data
//Variable are immutable by default
// If you want it mutable, declare it like this:
// let mut age = 37;
// Rust is a block-scoped language

pub fn run() {
    let _name = "Brad"; // Immutable
    let mut age = 37; // Mutable
    const ADDITION: i32 = 1;

    age = age + ADDITION;

    println!("Vars :");

    println!("Hello, my name is {} and I am {} years old.", _name, age);

    // Ownership of variables
    // In rust, variables can only have one owner.
    // String data type (Variable length) lives in the heap. A pointer is created in the stack and point to the heap
    // Integer, floating point etc (fixed size types) lives in the stack

    let mut outer_planet: String;

    {
        let inner_planet = String::from("Mercury");

        println!("Inner Planet is {}", inner_planet); // this is ok as inner_plabet still owns "Mercury"

        outer_planet = inner_planet.clone(); // This will work as we do a copy instead

        println!("Outer Planet is {}", outer_planet);

        outer_planet = inner_planet; // assign ownership to outer_planet. inner_planet no longer owns "Mercury"

        println!("Outer Planet is {}", outer_planet); // printing inner_planet will failed as it no longer owns "Mercury".
    }

    println!("Outer Planet is {}", outer_planet);

    // Example 2

    let i: i32;

    {
        let mut j = 1;

        println!("Value of j is {}", j);

        i = j; // We do not need to use clone() because primitive data type (with known size)
               // lives in Stack and not heap. a copy of j is created in the stack and assign to i.

        j += 1;

        println!("Value of i is {}", i);
        println!("Value of j after adding 1 is {}", j);
    }

    // function

    let v = 1;

    increment(v);

    let str = String::from("Hello");

    print_string(str);

    // println!("Value of str is {}", str); // this will failed unless we do print_string(str.clone())
    //                                          or get function to return the value back to the variable
    //                                          eg let str = print_string(str). print_string will be define as
    //                                          fn print_string(s: String) -> String { return (s);}
}

fn increment(mut i: i32) {
    i += 1; // the function is sent a copy of the data. so, this will not change original data.

    println!("Value of i in the function is  {}", i);
}

fn print_string(s: String) {
    println!("Value of s is {}", s);
}
// Scalar Types

// signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false
// and the unit type (), whose only possible value is an empty tuple: ()
//
// Compound Types

// arrays like [1, 2, 3]
// tuples like (1, true)
