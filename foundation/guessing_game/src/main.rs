
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The const value is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y in outer scope is: {y}");

    // let mut space = "    ";
    let space = "    ";
    let space = space.len();

    println!("Spaces: {space}");

    let eight_bit: u8 = 250;
    println!("Eight bit value: {eight_bit}");

    match 10u8.checked_add(eight_bit) {
        Some(sum) => println!("Sum is {sum}"),
        None => println!("Overflow!")
    };
}