use std::io;

fn main() {
    println!("Simple Calculator");
    println!("=================");

    // Get first number
    println!("Enter first number: ");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let num1: f64 = input1.trim().parse().expect("Invalid input");

    // Get operator
    println!("Enter operator (+, -, *, /): ");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read input");
    let operator = operator.trim();

    // Get second number
    println!("Enter second number: ");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let num2: f64 = input2.trim().parse().expect("Invalid input");

    // Calculate result
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Error: Invalid operator. Use +, -, *, or /");
            return;
        }
    };

    // Display result
    println!("\nResult: {} {} {} = {}", num1, operator, num2, result);
}
