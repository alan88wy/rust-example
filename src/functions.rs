pub fn run() {

    greeting("Hello", "James");

    let get_sum= add(5, 10);

    println!("The sum of {}", get_sum);

    // Closure

    let n3 = 10;

    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("The sum of add_sum : {}", add_sum(3,3));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(m:i32, n:i32) -> i32 {
    m + n
}