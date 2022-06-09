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
