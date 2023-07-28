use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operation {
    Add,
    Substract,
    Divide,
    Multiply,
    Power,
    Mod,
    Div,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operation::Add => write!(f, "+"),
            Operation::Substract => write!(f, "-"),
            Operation::Divide => write!(f, "/"),
            Operation::Multiply => write!(f, "*"),
            Operation::Power => write!(f, "^"),
            Operation::Mod => write!(f, "%"),
            Operation::Div => write!(f, "\\"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Int(i32),
    Float(f32),
    BinaryExpr {
        op: Operation,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryMinus(Box<Node>),
    Factorial(Box<Node>),
    Cos(Box<Node>),
    Sin(Box<Node>),
    Tg(Box<Node>),
    Ctg(Box<Node>),
    Log {
        value: Box<Node>,
        base: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Int(n) => write!(f, "{}", n),
            Node::Float(n) => write!(f, "{}", n),
            Node::UnaryMinus(rhs) => write!(f, "-{}", rhs),
            Node::Factorial(rhs) => write!(f, "{}!", rhs),
            Node::Cos(rhs) => write!(f, "cos({})", rhs),
            Node::Sin(rhs) => write!(f, "sin({})", rhs),
            Node::Tg(rhs) => write!(f, "tg({})", rhs),
            Node::Ctg(rhs) => write!(f, "ctg({})", rhs),
            Node::Log { value, base } => write!(f, "log{}({})", base, value),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}
