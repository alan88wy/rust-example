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

    // Enumerate

    let message = ['h', 'e', 'l', 'l', 'o'];

    for (idx, &item) in message.iter().enumerate() {
        println!("index {} has {}", idx, item);
    }

    // Nested loop & mutable element

    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("Print original matrix");

    for row in matrix {
        for i in row {
            print!("{}\t", i);
        }

        println!();
    }

    println!("adding 10 to each cell");
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10; // need to us *. Otherwise, will gives error.
            print!("{}\t", num);
        }
        println!();
    }

    println!("Print matrix after adding 10 to each cell");

    for row in matrix {
        for i in row {
            print!("{}\t", i);
        }

        println!();
    }
}
