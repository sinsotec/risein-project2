use std::io;
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                0.0
            }
        }
    }
}

fn main() {
    let x: f64;
    let y: f64;
    let result: f64;

    
    println!("Insert the first number:");
    x = input_number();
    println!("Insert the second number:");
    y = input_number();

    println!("Select the operation:");
    println!("Press 1 to ADD");
    println!("Press 2 to SUBTRACT");
    println!("Press 3 to MULTIPLY");
    println!("Press 4 to DIVIDE");
    
    match input_number() as u8 {
        1 => result = calculate(Operation::Add(x, y)),
        2 => result = calculate(Operation::Subtract(x, y)),
        3 => result = calculate(Operation::Multiply(x, y)),
        4 => result = calculate(Operation::Divide(x, y)),
        _ => result = 0.0
        
    };

    println!("The result is: {}", result);
    
}


fn input_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}