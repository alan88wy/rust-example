pub fn run() {
    
    let mut count = 0;

    // infinite loop

    println!("Loop:"); 

    loop {
        count += 1;
        println!("Number : {}", count);

        if count == 20 {
            break;
        }
    }

    // WHile loop (FizzBuzz)

    println!("While Loop:");

    count = 0;

    while count <= 100 {
        if count & 15 == 0 {
            println!("FizzBuzz");
        } else if count & 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    // For loop
    
    count = 0;

    println!("For Loop:");

    for _i in 0..100 {
        if count & 15 == 0 {
            println!("FizzBuzz");
        } else if count & 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }
}