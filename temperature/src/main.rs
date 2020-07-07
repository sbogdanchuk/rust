use std::io;

fn main() {
    println!("Temperature converter are welcomes you!");
    println!(
        "Please choose input system ('F' for Fahrenheit and 'C' for Celsius (default is F->C:"
    );

    let choise = loop {
        let mut choosen = String::new();

        io::stdin()
            .read_line(&mut choosen)
            .expect("Failed to read line");
        let res = choosen.trim().to_lowercase();

        if res == "f" || res == "c" || res == "q" {
            break res;
        } else {
            println!("c -> C->F");
            println!("f -> F->C");
            println!("q -> Quit");
        }
    };

    if choise == "q" {
        println!("Now quit, Bye!");
    } else if choise == "f" {
        println!("Fahrenheit->Celsius:");
        convert_f_c();
    } else {
        println!("Celsius->Fahrenheit:");
        convert_c_f();
    }
    println!("Your choise is {}", choise);
}

fn convert_f_c() {
    loop {
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Parse error: {}", err);
                continue;
            }
        };
        let res = (temp - 32.0) * 5.0 / 9.0;
        println!("{}F={}C", temp, res);
        break;
    }
}

fn convert_c_f() {
    loop {
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Parse error: {}", err);
                continue;
            }
        };
        let res = temp * 9.0 / 5.0 + 32.0;
        println!("{}C={}F", temp, res);
        break;
    }
}
