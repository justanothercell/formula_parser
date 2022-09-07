# Formula Parser

Simple but elegant formula parser in python with helpful error messages

````rs
fn main(){
    let input = "y * (x / π) + (5 * z * a)";
    let expr = match parser::parse(input) {
        Ok(expr) => expr,
        Err(error) => panic!("{}", error)
    };

    let vars = HashMap::from([
        ("x", 4f32),
        ("y", -3f32),
        ("z", 9f32),
        ("a", -1f32),
        ("π", PI)
    ]);

    println!("{:?}", expr);
    //Operation(Add, Operation(Mul, Variable("y"), Operation(Div, Variable("x"), Variable("π"))), Operation(Mul, Operation(Mul, Value(5.0), Variable("z")), Variable("a")))

    let expr_no_vars = expr.replace_vars(&vars);

    println!("{} = {}", expr, expr.eval(&vars));
    //y * x / π + 5 * z * a = -48.819717
    
    println!("{} = {}", expr_no_vars, expr_no_vars.eval(&HashMap::<&str, f32>::new()));
    // -3 * 4 / 3.1415927 + 5 * 9 * -1 = -48.819717
}
````

### TODO
- [x] Return result instead of panicking