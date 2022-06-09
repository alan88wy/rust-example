use std::mem;

pub fn run() {
    let numbers: Vec<i32> = vec![1,2,3,4,5];  // Cannot change item
    let mut arr: Vec<i32> = vec![1,2, 9, 3,4]; // Can change item
   
    println!("Vector (Resizable Array):");
    println!("{:?}", numbers);

    arr.push(10);
    arr.push(20);

    arr.sort();
    println!("{:?}", arr);

    arr.pop();
    println!("After popping one item : {:?}", arr);
    println!("Array occupies {:?} bytes", mem::size_of_val(&arr));

    // Get slice

    let slice: &[i32] = &numbers[0..2];

    println!("{:?}", slice);

    // Loop through vector
    println!("Looping vector");

    for x in arr.iter() {
        println!("{:?}", x);
    }

    // loop & mutate

    for x in arr.iter_mut() {
        *x *=2;
    }

    println!("{:?}", arr);

}