use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use Expr::*;
use Op::*;

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Operation(Op, Box<Expr>, Box<Expr>),
    Value(f32),
    Variable(String)
}

impl Expr {
    pub(crate) fn replace_vars(&self, vars: &HashMap<&str, f32>) -> Self{
        match self {
            Operation(op, box left, box right) => Operation(*op,
                                                            Box::from(left.replace_vars(vars)),
                                                            Box::from(right.replace_vars(vars))),
            Value(val) => Value(*val),
            Variable(var) => Value(*vars.get(var.as_str()).expect(format!("Variable '{}' does not exist!", var).as_str()))
        }
    }

    pub(crate) fn eval(&self, vars: &HashMap<&str, f32>) -> f32{
        match self {
            Operation(op, box left, box right) => op.eval(left, right, vars),
            Value(val) => *val,
            Variable(var) => *vars.get(var.as_str()).expect(format!("Variable '{}' does not exist!", var).as_str())
        }
    }

    fn format(&self, outer: Option<&Op>) -> String{
        match self {
            Operation(op, box left, box right) => match (op, outer) {
                (_, None | Some(Add | Sub)) => format!("{} {} {}", left.format(Some(op)), op, right.format(Some(op))),
                (Pow | Mul | Div, Some(Mul | Div)) => format!("{} {} {}", left.format(Some(op)), op, right.format(Some(op))),
                _ => format!("({} {} {})", left.format(Some(op)), op, right.format(Some(op))),
            },
            Value(val) => format!("{}", val),
            Variable(var) => format!("{}", var)
        }

    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format(None))
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow
}

impl Op {
    fn eval(&self, left: &Expr, right: &Expr, vars: &HashMap<&str, f32>) -> f32{
        match self {
            Add => left.eval(vars) + right.eval(vars),
            Sub => left.eval(vars) - right.eval(vars),
            Mul => left.eval(vars) * right.eval(vars),
            Div => left.eval(vars) / right.eval(vars),
            Pow => left.eval(vars).powf(right.eval(vars)),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Pow => "^"
        })
    }
}