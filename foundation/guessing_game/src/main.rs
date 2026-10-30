use std::io;
mod ownership;

// declaring constants
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn guessing_array_game() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter the index to access the value:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("THe value of the index {index} is: {element}");
}

fn main() {
    // variable and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The const value is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y in outer scope is: {y}");

    // let mut space = "    "; -- With mut the type cannot change, shadowing will not be possible like below
    let space = "    ";
    let space = space.len();

    println!("Spaces: {space}");

    // integer types
    let eight_bit: u8 = 250;
    println!("Eight bit value: {eight_bit}");

    // overflow control with match: we ave wrapping_*, checked_*, overflowing_* and saturating_* methods too
    match 10u8.checked_add(eight_bit) {
        Some(sum) => println!("Sum is {sum}"),
        None => println!("Overflow!"),
    };

    // Floating point numbers
    let x = 2.6;
    let y: f32 = 3.0;
    println!("x + y = {}", x + y);

    // Numberic operations
    let sum = 5 + 100;
    let multiplication = 45 * 2;
    let division = 52 / 5; // 52f64 or 52.0 makes things float else integer and it gets truncated
    let subtraction = 30 - 20;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!(
        "Sum is {sum}, multiplication is {multiplication}, division is {division}, subtraction is {subtraction}, truncated value is {truncated}, and remainder is {remainder}!"
    );

    // Boolean value
    let boolean_value = true;
    println!("The boolean value {boolean_value}");

    // character type
    let c = 'x';
    let z: char = 'Z';
    let heart_eyed_cat = '🥳';
    println!("{c}, {z}, {heart_eyed_cat}");

    // Compount types - Tuple Type and Array Type - Allows combining multiple values to gether.
    // 1. Tuple - once declared cannot grow and shrink in size, has a fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (x1, x2, x3) = tup;
    let five_hundred = tup.0; // taking value by index
    // let tupleWithNoValue: () = (); - A tuple with empty value is called Unit
    println!("Tuple values: {x1}, {x2}, {x3}, also: {five_hundred}");

    // 2. Array Type
    let a: [i32; 4] = [1, 2, 3, 4];
    let b = [3; 5]; // this allows us to create array with 5 elements all with value 3
    let array_val = a[0] + b[0];
    println!("{array_val}");
    guessing_array_game(); // part of function but was done to try array index bound error

    // functions
    // a simple function example is above guessing_array_game.
    parameterised_function(21, '😅');

    /*
     * STATEMENT AND EXPRESSIONS
     * - compared to other langugages rust has this differently
     * - Every line that does not return a value is a statement, and evaluating result that returns value is called expression.
     * - eg: let x = 1 is statement, but let x = (let y = 1) will throw error because we are assigning another statement to a statement. and a statement expects an expression not a statement.
     * - eg: let x = 1 + 1; now 1 + 1 is 2 and this returns the value 2 to x. X holds the value and not returns anything hence it is just a statement.
     * - Rust does have things like this unlike any other langugage x = y = 2.
     * - declaring function is a statement, calling a function is expression, calling macro is expression, a new scope block created with curly bracket is an expression.
     */
    let exp = {
        let y = 3;
        let z = 3;
        y + z
    };
    /*
     * In the above example if you see this exp is a scope expression that evaluates a result as 4 and returns it to exp variable.
     * Note: y + 1 does not have semi colon, and as soon as we have semi colon it will become an statement and not expression.
     */
    println!("{exp}");

    // Functions and Return Values
    let five = return_five();
    println!("{five}");

    // Control Flow
    condition_function(14);

    // Repetations with loops
    loops_function();

    // ownership
    ownership::ownership_rules();
}

/**
 * Parameters helps rust to know the exact type and avoid any kind of unwanted values.
 */
fn parameterised_function(x: i32, c: char) {
    println!("The given number and character is {x}, {c}")
}

/*
 * - Rust expression functions needs to have -> with type and a value to return without semicolon.
 * - The below approach is common but if you want rust to do early return you can use "return" keyword as well. 
 * - Both works in rust 
 */
fn return_five() -> i32 {
    5
}

/**
 * Unlike JS or some other langugage Rust does not convert a non boolean value to Boolean.
 * If condition expects the expression to be boolean
 */
fn condition_function(x: i32) {
    if x < 5 {
        println!("Condition was true");
    } else if x > 5 && x < 15 {
        println!("Condition is neither true or false");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 10 }; // value of the arms should be of same type else we get an error
    println!("{number}")
}

/**
 * 
 */
fn loops_function() {
    // loop keyword
    // loops until stopped using ctrl + c, and below code is commented so that we can execute other lines else this would never allow to go further
    // loop {
    //     println!("Again!");
    // }

    // break keyword
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    // Disambiguating with loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count {count}");

    // streamlining conditional loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Out of while loop");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is : {}", a[index]);
        index += 1;
    }

    /*
     * the above code is error prone, as if array size is reduced and forgot to change the manual size it would be an error 
     * for this reason we have for - loop,
     * also for is most used in rustacian, because it just allows you to do even things that we tried with while loop, given below
     * For loop is efficient and good for performance as in while we also compare indexes on every loop
     */
    for element in a {
        println!("{element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("Out of for loop")

}
