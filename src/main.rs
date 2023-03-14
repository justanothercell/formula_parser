#![feature(pattern, box_patterns)]

use std::collections::HashMap;
use std::f32::consts::{E, PI};
use std::io::Write;

mod parser;
mod expression;

fn main() {
    let mut vars = HashMap::from([
        ("π".to_string(), PI),
        ("pi".to_string(), PI),
        ("e".to_string(), E)
    ]);
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("could not read input");
        if let Some((var, expr_in)) = input.split_once(":=") {
            let var = var.trim();
            match parser::parse(expr_in) {
                Ok(expr) => {
                    let res = expr.eval(&vars);
                    println!("{} := {}\n{} = {}", var, expr, var, res);
                    vars.insert(var.to_string(), res);
                },
                Err(error) => println!("Err: {error}")
            };

        } else {
            match parser::parse(&input) {
                Ok(expr) => {
                    let res = expr.eval(&vars);
                    println!("{} = {}", expr, res);
                },
                Err(error) => println!("Err: {error}")
            };
        }
    }
}
