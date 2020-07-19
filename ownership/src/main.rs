fn main() {
    let s = String::from("hello");
    takes_ownership(s.clone());
    println!("S after clone {}", s);
    let x = 5;
    makes_copy(x);
    println!("X after copy {}", x);

    main3();
    main4();
    main5();
    let s = String::from("hello world");
    println!("First word: {}", first_word(&s));
    main6();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn main2() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn main3() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main4() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str((", world"));
}

fn main5() {
    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..];
    println!("{}, {}", hello, world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main6() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}
