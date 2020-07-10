use std::io;

fn main() {
    println!("Fibonacci counter ('q' for quit)");
    loop {
        println!("Please input desired fibonacci member");
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.to_lowercase().trim() == "q" {
            println!("Bye!");
            break;
        }
        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Parse error: {}", err);
                continue;
            }
        };
        if n > 92 {
            println!("Too big number! n<=92");
            continue;
        }
        fibo(n);
    }
}

fn fibo(n: u64) {
    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    for _ in 0..n {
        let temp = cur;
        cur = prev + cur;
        prev = temp;
    }
    println!("{}th fibonacci is {}", n, prev);
}
