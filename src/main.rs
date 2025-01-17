use std::io::Write;
use debug_print::debug_print;

fn main() {
    let mut buf = std::string::String::new();
    let mut num1: i64 = 0;
    let mut num2: i64 = 0;
    let mut result: i64 = 0;
    println!("Welcome to {}", env!("CARGO_PKG_NAME"));

    print!("Enter number 1: ");
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    num1 = match buf.parse::<i64>() {
        Ok(num) => num,
        Err(e) => panic!("{}", e)
    };
    debug_print!("Read number: {}\n", num1);
    buf.clear();

    print!("Enter number 2: ");
    std::io::stdout().flush().unwrap();
    let _ = std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    num2 = match buf.parse::<i64>() {
        Ok(num) => num,
        Err(e) => panic!("{}", e)
    };
    debug_print!("Read number: {}\n", num2);
    
    let term = console::Term::stdout();
    let mut result: i64 = 0;
    print!("Specify operation:");
    std::io::stdout().flush().unwrap();
    match match term.read_char() {
        Ok(ch) => ch,
        Err(e) => panic!("{}", e)
    } {
        '+' => result = num1 + num2,
        '-' => result = num1 - num2,
        '*' => result = num1 * num2,
        '/' => result = num1 / num2,
        '&' => result = num1 & num2,
        '|' => result = num1 | num2,
        '^' => result = num1 ^ num2,
        '%' => result = num1 % num2,
        'r' => {
            println!("Result: {}", (num1 as f64).sqrt());
            std::process::exit(0);
        }
        'p' => result = num1.pow(num2 as u32),
        _   => panic!("Unsupported operation specified!")
    }
    println!("\nResult: {}", result);
}
