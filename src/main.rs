use std::env;
use std::iter::Peekable;
use std::str::Chars;

fn add(buffer: &mut Peekable<Chars>, value: i32) -> i32 {
    value + eval_expr(buffer, 0, false)
}

fn subtract(buffer: &mut Peekable<Chars>, value: i32) -> i32 {
    value - eval_expr(buffer, 0, false)
}

fn multiply(buffer: &mut Peekable<Chars>, value: i32) -> i32 {
    let factor = value * pow_eval_expr(buffer);
    eval_expr(buffer, factor, true)
}

fn divide(buffer: &mut Peekable<Chars>, value: i32) -> i32 {
    let quotient = value / pow_eval_expr(buffer);
    eval_expr(buffer, quotient, true)
}

fn power(buffer: &mut Peekable<Chars>, value: i32) -> i32 {
    let pow = i32::pow(value, parse_number(buffer) as u32);
    eval_expr(buffer, pow, true)
}

fn parse_number(buffer: &mut Peekable<Chars>) -> i32 {
    let mut is_number = true;
    let mut factor = 1;
    let mut number_str = String::new();
    let mut token = buffer.peek();
    if !token.is_none() && *token.unwrap() == '-' {
        factor = -1;
        buffer.next();
    }
    while is_number {
        token = buffer.peek();
        if token.is_none() {
            break;
        }
        let token_char = token.unwrap();
        if token_char.is_numeric() {
            number_str = number_str + token_char.to_string().as_str(); // Yeah yeah ugly I know
            buffer.next();
        } else { is_number = false; }
    }
    if number_str.is_empty() {
        return 0;
    }
    number_str.parse::<i32>().unwrap() * factor
}

fn pow_eval_expr(buffer: &mut Peekable<Chars>) -> i32 {
    let value = parse_number(buffer);
    let token = buffer.next();
    match token {
        Some('^') => {power(buffer, value)},
        None => {value}
        Some(_) => {value}
    }
}

fn eval_expr(buffer: &mut Peekable<Chars>, left_value: i32, has_left_value: bool) -> i32 {
    let value = if !has_left_value { parse_number(buffer) } else {left_value};
    let token = buffer.next();
    match token {
        Some('+') => {add(buffer, value)},
        Some('-') => {subtract(buffer, value)},
        Some('*') => {multiply(buffer, value)},
        Some('/') => {divide(buffer, value)},
        Some('^') => {power(buffer, value)},
        None => {value}
        Some(_) => {value}
    }
}


fn main() {
    let expr = "3*2^3".to_string().replace(" ", "");
    let mut expr_peekable = expr.chars().peekable();
    println!("{}={}", expr, eval_expr(& mut expr_peekable, 0, false));

    for argument in env::args() {
        let expr = argument.replace(" ", "");
        let mut expr_peekable = expr.chars().peekable();
        println!("{}={}", expr, eval_expr(& mut expr_peekable, 0, false));
    }
}
