use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use super::sdb::Expr;


#[derive(Parser)]
#[grammar = "./monitor/grammar.pest"]
pub enum SDB {}


pub fn get_expr(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr);
    let mut iter = i.into_inner();
    let mut r = get_expr_relational(iter.next().unwrap());
    while let Some((op, rhs)) = get_expr_expend(&mut iter) {
        r = match op {
            '&' => Expr::And(Box::new(r), Box::new(rhs)),
            '|' => Expr::Or(Box::new(r), Box::new(rhs)),
            _ => unreachable!(),
        };
    }
    r
}

fn get_expr_expend(i: &mut Pairs<Rule>) -> Option<(char, Expr)> {
    let op = i.next()?;
    let rhs = get_expr_relational(i.next().unwrap());
    let op = match op.as_str() {
        "&&"    => '&',
        "||"     => '|',
        _ => unreachable!()
    };
    Some((op, rhs))
}

fn get_expr_relational(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr_relational);
    let mut iter = i.into_inner();
    let mut r = get_expr_binary_level1(iter.next().unwrap());
    while let Some((op, rhs)) = get_expr_relational_expend(&mut iter) {
        r = match op {
            "<="    => Expr::Leq(Box::new(r), Box::new(rhs)),
            "<"     => Expr::Lt(Box::new(r), Box::new(rhs)),
            ">="    => Expr::Geq(Box::new(r), Box::new(rhs)),
            ">"     => Expr::Gt(Box::new(r), Box::new(rhs)),
            "=="    => Expr::Eq(Box::new(r), Box::new(rhs)),
            "!="    => Expr::Ne(Box::new(r), Box::new(rhs)),
            _ => unreachable!(),
        };
    }
    r
}

fn get_expr_relational_expend(i: &mut Pairs<Rule>) -> Option<(&'static str, Expr)> {
    let op = i.next()?;
    let rhs = get_expr_binary_level1(i.next().unwrap());
    // /*
    let op = match op.as_str() {
        "<="    => "<=",
        "<"     => "<",
        ">="    => ">=",
        ">"     => ">",
        "=="    => "==",
        "!="    => "!=",
        _ => unreachable!()
    };
    //  */
    Some((op, rhs))
}

fn get_expr_binary_level1(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr_binary_level1);
    let mut iter = i.into_inner();
    let mut r = get_expr_binary_level2(iter.next().unwrap());
    while let Some((op, rhs)) = get_expr_binary_level1_expend(&mut iter) {
        r = match op {
            '+' => Expr::Add(Box::new(r), Box::new(rhs)),
            '-' => Expr::Sub(Box::new(r), Box::new(rhs)),
            _ => unreachable!(),
        };
    }
    r
}

fn get_expr_binary_level1_expend(i: &mut Pairs<Rule>) -> Option<(char, Expr)> {
    let op = i.next()?;
    let rhs = get_expr_binary_level2(i.next().unwrap());
    let op = op.as_str().chars().next().unwrap();
    Some((op, rhs))
}


fn get_expr_binary_level2(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr_binary_level2);
    let mut iter = i.into_inner();
    let mut r = get_expr_binary_level3(iter.next().unwrap());
    while let Some((op, rhs)) = get_expr_binary_level2_expend(&mut iter) {
        r = match op {
            '*' => Expr::Mul(Box::new(r), Box::new(rhs)),
            '/' => Expr::Div(Box::new(r), Box::new(rhs)),
            '%' => Expr::Mod(Box::new(r), Box::new(rhs)),
            _ => unreachable!(),
        };
    }
    r
}

fn get_expr_binary_level2_expend(i: &mut Pairs<Rule>) -> Option<(char, Expr)> {
    let op = i.next()?;
    let rhs = get_expr_binary_level3(i.next().unwrap());
    let op = op.as_str().chars().next().unwrap();
    Some((op, rhs))
}

fn get_expr_binary_level3(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr_binary_level3);
    let mut iter = i.into_inner();
    let expr = iter.next().unwrap();
    if let Some(_) = iter.next() {
        let expr2 = iter.next().unwrap();
        Expr::Pow(Box::new(get_expr_binary_level3(expr)), Box::new(get_expr_binary_level3(expr2)))
    } else {
        get_expr_unray(expr)
    }
}

fn get_expr_unray(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr_unary);
    let mut iter = i.into_inner();
    let expr = iter.next().unwrap();
    if expr.as_rule() == Rule::unary_op {
        let op = expr.as_str();
        let expr = iter.next().unwrap();
        match op {
            "*" => Expr::Deref(Box::new(get_expr_unray(expr))),
            _ => panic!("unary operator not implemented: {}", op),
        }
    } else {
        get_expr_atom(expr)
    }
}

fn get_expr_atom(i: Pair<Rule>) -> Expr {
    debug_assert_eq!(i.as_rule(), Rule::expr_atom);
    let i = i.into_inner().next().unwrap();
    match i.as_rule() {
        Rule::expr => get_expr(i),
        Rule::reg => Expr::Reg(get_reg(i)),
        Rule::number => Expr::Num(get_number(i)),
        _ => unreachable!(),
    }
}

fn get_reg(i: Pair<Rule>) -> String {
    debug_assert_eq!(i.as_rule(), Rule::reg);
    let mut iters = i.into_inner();
    iters.next();
    let r = iters.next().unwrap();
    debug_assert!(r.as_rule() == Rule::id || r.as_rule() == Rule::number);
    r.as_str().to_string()
}

fn get_id(i: Pair<Rule>) -> String {
    debug_assert_eq!(i.as_rule(), Rule::id);
    i.as_str().to_string()
}

fn get_number(i: Pair<Rule>) -> u64 {
    debug_assert_eq!(i.as_rule(), Rule::number);
    let s = i.as_str();
    if s.chars().next().unwrap() == '-' {
        s.parse::<i64>().unwrap() as u64
    } else {
        s.parse::<u64>().unwrap()
    }
}


#[test]
fn test_parser() {
    let mut r = SDB::parse(Rule::expr, "1+1").unwrap();
    let r = r.next().unwrap();
    let r = get_expr(r);
    assert_eq!(r, Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(1))));
}

#[test]
fn test_parser1() {
    let mut r = SDB::parse(Rule::expr, "*114514").unwrap();
    let r = r.next().unwrap();
    let r = get_expr(r);
    assert_eq!(r, Expr::Deref(Box::new(Expr::Num(114514))));
}

#[test]
fn test_parser2() {
    let mut r = SDB::parse(Rule::expr, "1 + (3 - 4) * 2").unwrap();
    let r = r.next().unwrap();
    let r = get_expr(r);
    assert_eq!(r, Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Mul(Box::new(Expr::Sub(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))), Box::new(Expr::Num(2))))));
}

