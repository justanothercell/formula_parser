#![feature(pattern, box_patterns)]

use std::collections::HashMap;
use std::f32::consts::PI;

mod parser;
mod expression;

fn main() {
    let input = "y * (x / π) + (5 * z * a)";
    let expr = parser::parse(input);

    let vars = HashMap::from([
        ("x", 4f32),
        ("y", -3f32),
        ("z", 9f32),
        ("a", -1f32),
        ("π", PI)
    ]);

    println!("{:?}", expr);

    let expr_no_vars = expr.replace_vars(&vars);

    println!("{} = {}", expr, expr.eval(&vars));

    println!("{} = {}", expr_no_vars, expr_no_vars.eval(&HashMap::<&str, f32>::new()));
}
