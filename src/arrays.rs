use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Cannot change item
    let mut arr: [i32; 5] = [1, 2, 9, 3, 4]; // Can change item

    println!("Array :");

    println!("{:?}", numbers);

    arr[3] = 0; // Can change because of mutable array
    arr.sort();
    println!("{:?}", arr);

    println!("Array occupies {:?} bytes", mem::size_of_val(&arr));

    // Get slice

    let slice: &[i32] = &numbers[0..2];

    println!("{:?}", slice);

    // All elements can be initialized to the same value
    // Here we initialized array with 20 element of 0
    let ys: [i32; 20] = [0; 20];

    println!("Array : {:?}", ys);
}
