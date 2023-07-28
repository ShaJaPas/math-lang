use std::f32::consts::{E, PI};

use crate::ast::{Node, Operation};
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc::Left, Op, PrattParser},
    Parser,
};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

pub fn parse(source: &str) -> std::result::Result<Node, Box<pest::error::Error<Rule>>> {
    let mut pairs = CalcParser::parse(Rule::Program, source)?;
    let parser = PrattParser::new()
        .op(Op::infix(Rule::Add, Left) | Op::infix(Rule::Subtract, Left))
        .op(Op::infix(Rule::Multiply, Left)
            | Op::infix(Rule::Divide, Left)
            | Op::infix(Rule::Mod, Left)
            | Op::infix(Rule::Div, Left))
        .op(Op::infix(Rule::Power, Left))
        .op(Op::postfix(Rule::Factorial))
        .op(Op::prefix(Rule::UnaryMinus));
    let ast = build_ast(&parser, pairs.next().unwrap().into_inner());
    Ok(ast)
}

fn build_ast(parser: &PrattParser<Rule>, pairs: Pairs<Rule>) -> Node {
    parser
        .map_primary(|primary| match primary.as_rule() {
            Rule::Int => Node::Int(primary.as_str().parse().unwrap()),
            Rule::Float => Node::Float(primary.as_str().parse().unwrap()),
            Rule::PI => Node::Float(PI),
            Rule::E => Node::Float(E),
            Rule::Cos => Node::Cos(Box::new(build_ast(parser, primary.into_inner()))),
            Rule::Sin => Node::Sin(Box::new(build_ast(parser, primary.into_inner()))),
            Rule::Tg => Node::Tg(Box::new(build_ast(parser, primary.into_inner()))),
            Rule::Ctg => Node::Ctg(Box::new(build_ast(parser, primary.into_inner()))),
            Rule::Log => {
                let mut primary = primary.into_inner();
                let value = primary.next().unwrap();
                let base = primary.next().unwrap();
                Node::Log {
                    value: Box::new(build_ast(parser, value.into_inner())),
                    base: Box::new(build_ast(parser, base.into_inner())),
                }
            }
            Rule::Expr => build_ast(parser, primary.into_inner()),
            rule => unreachable!("expected Term, found: {rule:?}"),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::Add => Operation::Add,
                Rule::Subtract => Operation::Substract,
                Rule::Multiply => Operation::Multiply,
                Rule::Divide => Operation::Divide,
                Rule::Power => Operation::Power,
                Rule::Mod => Operation::Mod,
                Rule::Div => Operation::Div,
                rule => unreachable!("expected Operation, found: {rule:?}"),
            };
            Node::BinaryExpr {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::UnaryMinus => Node::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .map_postfix(|rhs, op| match op.as_rule() {
            Rule::Factorial => Node::Factorial(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}
