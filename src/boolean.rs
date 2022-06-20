pub fn run() {
    let a = true;
    let b = false;

    println!("a is {} and b is {} ", a, b);
    println!("Not a is {} ", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {} ", a | b);
    println!("a XOR b is {} ", a ^ b);

    let c = (a ^ b) | (a & b);
    println!("(a ^ b) | (a & b) = {}", c);
}
