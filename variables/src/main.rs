use std::mem::size_of;

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
    let spaces = 333;
    println!("spaces: {}", spaces);

    println!("isize: {}", size_of::<isize>());
    println!("f64 size: {}", size_of::<f64>());
    println!("is 64-bit ?: {}", size_of::<i64>() == 64 / 8);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "{} {} {} {} {}",
        sum, difference, product, quotient, remainder
    );
    let heart_eyed_cat = '😻';
    println!("{}, len:{}", heart_eyed_cat, heart_eyed_cat.len_utf8());

    //TUPLES

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;

    println!("The value of y is: {}", y);
    println!("The value of first element of tup: {}", tup.0);

    //Arrays

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    println!("Array init with 3th: {:?}", a);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let index = 0;
    let element = a[index];
    println!("The value of element is: {}", element);

    another_function();
}

fn another_function() {
    println!("Another function.");
}
