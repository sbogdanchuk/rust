fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    main2();
    main3();
    println!("five fn: {}", five());

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn main2() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("x:{}, y:{}", x, y);
}

fn main3() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
