fn main() {
    // integer addition
    println!("{} + {} = {}", 1, 1, 1 + 1);

    // integer subtraction
    println!("{} - {} = {}", 1, 2, 1 - 2);

    // boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operations
    let x = 0b0011;
    let y = 0b0101;

    println!("{:04b} AND {:04b} = {:04b}", x, y, x & y);
    println!("{:04b} OR {:04b} = {:04b}", x, y, x | y);
    println!("{:04b} XOR {:04b} = {:04b}", x, y, x ^ y);
    println!("1 << 5 = {}", 1 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2);

    // underscores for better readability
    assert_eq!(1_000_000, 10_00_000);
}
