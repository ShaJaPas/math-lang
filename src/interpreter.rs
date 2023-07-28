use crate::operations::Op;
use crate::{ast::Operation, Compile, Node};

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<f32, String>;

    fn from_ast(ast: Node) -> Self::Output {
        Eval::eval(&ast)
    }
}

struct Eval;

impl Eval {
    pub fn eval(node: &Node) -> Result<f32, String> {
        match node {
            Node::Int(n) => Ok(*n as f32),
            Node::Float(n) => Ok(*n),
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = Self::eval(lhs);
                let rhs_ret = Self::eval(rhs);

                match op {
                    Operation::Add => Ok(Op::add(lhs_ret?, rhs_ret?)),
                    Operation::Substract => Ok(Op::substract(lhs_ret?, rhs_ret?)),
                    Operation::Divide => Op::divide(lhs_ret?, rhs_ret?),
                    Operation::Multiply => Ok(Op::multiply(lhs_ret?, rhs_ret?)),
                    Operation::Power => Ok(Op::power(lhs_ret?, rhs_ret?)),
                    Operation::Mod => Op::modulus(lhs_ret?, rhs_ret?),
                    Operation::Div => Op::div(lhs_ret?, rhs_ret?),
                }
            }
            Node::UnaryMinus(rhs) => {
                let rhs_ret = Self::eval(rhs)?;
                Ok(Op::unary_minus(rhs_ret))
            }
            Node::Factorial(rhs) => {
                let rhs_ret = Self::eval(rhs)?;
                Op::factorial(rhs_ret)
            }
            Node::Sin(rhs) => {
                let rhs_ret = Self::eval(rhs)?;
                Ok(Op::sin(rhs_ret))
            }
            Node::Cos(rhs) => {
                let rhs_ret = Self::eval(rhs)?;
                Ok(Op::cos(rhs_ret))
            }
            Node::Tg(rhs) => {
                let rhs_ret = Self::eval(rhs)?;
                Ok(Op::tg(rhs_ret))
            }
            Node::Ctg(rhs) => {
                let rhs_ret = Self::eval(rhs)?;
                Ok(Op::ctg(rhs_ret))
            }
            Node::Log { value, base } => {
                let value = Self::eval(value)?;
                let base = Self::eval(base)?;
                Op::log(value, base)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Interpreter::from_source("10 + 2").unwrap().unwrap(), 12.0);
        assert_eq!(Interpreter::from_source("1 + -2").unwrap().unwrap(), -1.0);
        assert_eq!(
            Interpreter::from_source("2 + (2 - 1)").unwrap().unwrap(),
            3.0
        );
        assert_eq!(
            Interpreter::from_source("(2 + 3) - 1").unwrap().unwrap(),
            4.0
        );
        assert_eq!(
            Interpreter::from_source("1 + ((2 + 3) - (2 + 3))")
                .unwrap()
                .unwrap(),
            1.0
        );
        assert_eq!(
            Interpreter::from_source("(2) - (3)").unwrap().unwrap(),
            -1.0
        );
    }

    #[test]
    fn float() {
        assert_eq!(
            Interpreter::from_source("(2.5) - (3)").unwrap().unwrap(),
            -0.5
        );
        assert_eq!(
            Interpreter::from_source("-2.5 + 2.5").unwrap().unwrap(),
            0.0
        );
        assert_eq!(
            Interpreter::from_source("-2.5 - (3.2)").unwrap().unwrap(),
            -5.7
        );
        assert_eq!(
            Interpreter::from_source("(2.5) \\ 15").unwrap().unwrap(),
            0.0
        );
        assert_eq!(
            Interpreter::from_source("(64) ^ (1/3)").unwrap().unwrap(),
            4.0
        );
    }
}
