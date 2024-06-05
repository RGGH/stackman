use stackman::Calculator;
use std::{
    io::{self, Write},
};

fn main() {
    let mut calc = Calculator::new();
    println!("\nWelcome to stackman\n");

    loop {
        print!("> ");
        // Flushing the output - force any
        // buffered output to be written out immediately.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let tokens: Vec<&str> = input.split_whitespace().collect();

        for token in tokens {
            match token {
                "add" => calc.add(),
                "sub" => calc.sub(),
                "equal" => {
                    if let Some(result) = calc.equal() {
                        println!("Result {}", result);
                    } else {
                        println!("No result available");
                    }
                }
                "equal_verify" => {
                    if let Some(result) = calc.equal_verify() {
                        println!("Equal! - Result {}", result);
                    } else {
                        println!("False - Not Equal");
                    }
                }
                "dup" => calc.dup(),
                num_str => {
                    if let Ok(num) = num_str.parse::<i32>() {
                        calc.push(num);
                    } else {
                        println!("Invalid input: {}", num_str);
                    }
                }
            }
        }
    }
}
