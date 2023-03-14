use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::expression::{ Expr, Op };

pub(crate) fn parse(input: &str) -> Result<Expr, ParseError>{
    let mut input_iter = ParserIter::new(input);
    let here = input_iter.here();
    return parse_scope(&mut input_iter, &here, false);
}

fn parse_scope(input_iter: &mut ParserIter, start: &Lok, is_inner: bool) -> Result<Expr, ParseError>{
    let mut tokens = Vec::<Token>::new();
    let mut finished_with_bracket = false;
    while let Some(&c) = input_iter.peek() {
        if c.is_whitespace() {
            input_iter.next();
        }
        else if c == ')' {
            input_iter.next();
            finished_with_bracket = true;
            break;
        }
        else if c == '(' {
            input_iter.next();
            tokens.push(Token::Expr(parse_scope(input_iter, &input_iter.here(), true)?, input_iter.here()));
        }
        else if c.is_ascii_digit() {
            tokens.push(Token::Expr(parse_value(input_iter, &input_iter.here())?, input_iter.here()));
        }
        else if c.is_alphabetic() {
            tokens.push(Token::Expr(parse_variable(input_iter, &input_iter.here()), input_iter.here()));
        }
        else {
            match c {
                '+' | '-' => tokens.push(Token::DashOp(c.to_string(), input_iter.here())),
                '*' | '/' => tokens.push(Token::DotOp(c.to_string(), input_iter.here())),
                '^' => tokens.push(Token::PowOp(c.to_string(), input_iter.here())),
                _ => return input_iter.here().error(format!("Invalid token '{}'", c))
            }
            input_iter.next();
        }
    }

    if is_inner && !finished_with_bracket {
        return start.error("Missing closing bracket for this opening bracket".to_string());
    }

    macro_rules! combine_tokens {
        ($op_type: ident) => {
            {
                let mut tokens_iter = tokens.iter();
                let mut combined_tokens = Vec::<Token>::new();
                while let Some(tok) = tokens_iter.next() {
                    match tok {
                        Token::$op_type(op, lok) => {
                           if let (Some(left), Some(right)) = (combined_tokens.pop(), tokens_iter.next()){
                                if let (Token::Expr(expr_l, _), Token::Expr(expr_r, _)) = (left.clone(), right.clone()) {
                                    combined_tokens.push(Token::Expr(Expr::Operation(match op.as_str() {
                                        "+" => Op::Add,
                                        "-" => Op::Sub,
                                        "*" => Op::Mul,
                                        "/" => Op::Div,
                                        "^" => Op::Pow,
                                        s => panic!("Invalid op symbol '{}'", s)
                                    }, Box::from(expr_l), Box::from(expr_r)), lok.clone()));
                                }
                                else{
                                    return lok.error(format!("Invalid tokens for operation '{}': '{}' and '{}'", op, left, right));
                                }
                            }
                            else{
                                return lok.error(format!("Operation '{}' requires expressions on both sides", op));
                            }
                        },
                        t => combined_tokens.push(t.clone())
                    }
                }
                combined_tokens
            }
        };
    }


    tokens = combine_tokens!(PowOp);
    tokens = combine_tokens!(DotOp);
    tokens = combine_tokens!(DashOp);

    if tokens.len() == 1 {
        if let Some(Token::Expr(expr, _)) = tokens.pop(){
            return Ok(expr);
        }
    }
    if tokens.len() > 1 {
        return tokens.pop().unwrap().lok().error("Found more than one expressions without operator in between!".to_string());
    }

    Ok(Expr::Value(0.0))
}

#[derive(Clone)]
struct ParserIter<'a> {
    original: String,
    iter: Peekable<Chars<'a>>,
    index: usize
}

impl ParserIter<'_> {
    fn new(input: &str) -> ParserIter {
        ParserIter {
            original: input.clone().to_string(),
            iter: input.chars().peekable(),
            index: 0
        }
    }

    fn next(&mut self) -> Option<char>{
        self.index += 1;
        self.iter.next()
    }

    fn peek(&mut self) -> Option<&char>{
        self.iter.peek()
    }

    fn here(&self) -> Lok{
        Lok {
            original: self.original.clone(),
            index: self.index
        }
    }
}

fn parse_value(input_iter: &mut ParserIter, start: &Lok) -> Result<Expr, ParseError>{
    let mut val = String::new();
    let mut had_dot = false;
    while let Some(c) = input_iter.peek() {
        if c.is_ascii_digit() {
            val.push(*c);
            input_iter.next();
        }
        else if c == &'.' {
            if !had_dot {
                val.push(*c);
                had_dot = true;
                input_iter.next();
            }
            else{
                input_iter.next();
                return input_iter.here().error("Encountered decimal point for a second time in this number!".to_string());
            }
        }
        else{
            break;
        }
    }
    return if let Ok(v) = f32::from_str(val.as_str()) {
        Ok(Expr::Value(v))
    } else {
        start.error(format!("Could not parse value {}", val))
    }
}

fn parse_variable(input_iter: &mut ParserIter, start: &Lok) -> Expr{
    let mut var = String::new();
    while let Some(c) = input_iter.peek() {
        if c.is_alphanumeric() {
            var.push(*c);
            input_iter.next();
        }
        else{
            break;
        }
    }
    return Expr::Variable(var);
}

#[derive(Debug, Clone)]
struct Lok {
    original: String,
    index: usize,
}

impl Display for Lok {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}^", self.original, " ".repeat(if self.index > 0 { self.index - 1 } else { 0 }))
    }
}

impl Lok {
    fn error(&self, msg: String) -> Result<Expr, ParseError> {
        Err(ParseError(format!("{}\n\n{}", msg, self)))
    }
}

#[derive(Debug)]
pub(crate) struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
enum Token {
    Expr(Expr, Lok),
    PowOp(String, Lok),
    DotOp(String, Lok),
    DashOp(String, Lok)
}

impl Token {
    fn lok(&self) -> &Lok {
        match self {
            Token::Expr(_, lok) => lok,
            Token::PowOp(_, lok) => lok,
            Token::DotOp(_, lok) => lok,
            Token::DashOp(_, lok) => lok
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Token::Expr(expr, _) => format!("{}", expr),
            Token::DotOp(op, _) | Token::PowOp(op, _) | Token::DashOp(op, _) => op.to_string(),
        })
    }
}