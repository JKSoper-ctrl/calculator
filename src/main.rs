use std::io;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    let number1: f64 = loop {
        println!("Provide first number:");

        let number_input : String = get_input();

        match number_input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                eprintln!("Not valid number. Try again.");
                continue;
            },
        }
    };
    
    let operator: Operator = loop {
        println!("Provide operator (+, -, *, /):");

        let input_operator: String = get_input();

        match input_operator.trim() {
            "+" => break Operator::Add,
            "-" => break Operator::Subtract,
            "*" => break Operator::Multiply,
            "/" => break Operator::Divide,
            _ => {
                eprintln!("Not valid operator. Try again.");
                continue;
            }
        };
    };

    let number2: f64 = loop {
        println!("Provide first number:");

        let number_input : String = get_input();

        match number_input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                eprintln!("Not valid number. Try again.");
                continue;
            },
        }
    };

    let result: f64 = match operator {
        Operator::Add => number1 + number2,
        Operator::Subtract => number1 - number2,
        Operator::Multiply => number1 * number2,
        Operator::Divide => number1 / number2,
    };
    println!("Result:");
    println!("{}", result.to_string());
}

fn get_input() -> String {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read input");
    input_string.to_string()
}