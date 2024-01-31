use std::io;

fn main() {
    println!("Simple Rust Calculator");

    loop {
        println!("Enter an expression (e.g., 2 + 3):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting calculator. Goodbye!");
            break;
        }

        match calculate(input) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn calculate(expression: &str) -> Result<f64, &'static str> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() != 3 {
        return Err("Invalid expression. Please enter an expression with two operands and an operator.");
    }

    let operand1: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid first operand. Please enter a valid number."),
    };

    let operator = tokens[1];

    let operand2: f64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid second operand. Please enter a valid number."),
    };

    let result = match operator {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" => operand1 * operand2,
        "/" => {
            if operand2 == 0.0 {
                return Err("Cannot divide by zero.");
            }
            operand1 / operand2
        }
        _ => return Err("Invalid operator. Please use +, -, *, or /."),
    };

    Ok(result)
}
