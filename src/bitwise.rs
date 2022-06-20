pub fn run() {
    let mut value = 0b1111_0101u8;

    println!("{}", value);
    println!("{:08b}", value);

    // Bitwise operation

    // Not Operator

    // value = !value;
    println!("NOT Operator");
    println!("Before NOT : {:08b}", value);
    println!("Not Result : {:08b}", !value);

    // AND Operator

    // value = 0b1111_0101u8;
    println!("AND Operator");
    println!("Before AND : {:08b}", value);
    println!("Value use: : {:08b}", 0b1111_0111);
    println!("AND Result : {:08b}", value & 0b1111_0111);

    // AND Operator

    println!("Compare Bits");
    println!("Before AND : {:08b}", value);
    println!("Value use: : {:08b}", 0b1111_0111);
    value = value & 0b1111_0111;
    println!("Result     : {:08b}", value);
    println!("bit 6 is   : {:08b}", value & 0b0100_0000);

    // OR Operator
    println!("OR Operator");
    println!("Before OR : {:08b}", value);
    println!("Value use : {:08b}", 0b0100_0000);
    println!("OR Result : {:08b}", value | 0b0100_0000);

    // XOR
    println!("XOR Operator");
    println!("Before XOR : {:08b}", value);
    println!("Value use  : {:08b}", 0b0101_0101);
    println!("XOR Result : {:08b}", value ^ 0b0101_0101);

    // Bitwise shift
    println!("Shift Operator");
    println!("Before Shift     : {:08b}", value);
    println!("Left Shift by 4  : {:08b}", value << 4);
    println!("Right Shift by 4 : {:08b}", value >> 4);
}
