use bincode::{config, Decode, Encode};

use crate::{
    ast::{Node, Operation},
    Compile,
};

use super::opcode::OpCode;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Bytecode {
    pub instructions: Vec<OpCode>,
}

impl Bytecode {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&self) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).unwrap()
    }

    fn add_instruction(&mut self, op_code: OpCode) {
        self.instructions.push(op_code);
    }

    fn interpret_node(&mut self, node: Node) {
        match node {
            Node::Int(n) => {
                self.add_instruction(OpCode::OpPush(n as f32));
            }
            Node::Float(n) => {
                self.add_instruction(OpCode::OpPush(n));
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                self.interpret_node(*lhs);
                self.interpret_node(*rhs);

                match op {
                    Operation::Add => self.add_instruction(OpCode::OpAdd),
                    Operation::Substract => self.add_instruction(OpCode::OpSubstract),
                    Operation::Divide => self.add_instruction(OpCode::OpDivide),
                    Operation::Multiply => self.add_instruction(OpCode::OpMultiply),
                    Operation::Power => self.add_instruction(OpCode::OpPower),
                    Operation::Mod => self.add_instruction(OpCode::OpMod),
                    Operation::Div => self.add_instruction(OpCode::OpDiv),
                };
            }
            Node::UnaryMinus(rhs) => {
                self.interpret_node(*rhs);
                self.add_instruction(OpCode::OpUnaryMinus);
            }
            Node::Factorial(rhs) => {
                self.interpret_node(*rhs);
                self.add_instruction(OpCode::OpFactorial);
            }
            Node::Sin(rhs) => {
                self.interpret_node(*rhs);
                self.add_instruction(OpCode::OpSin);
            }
            Node::Cos(rhs) => {
                self.interpret_node(*rhs);
                self.add_instruction(OpCode::OpCos);
            }
            Node::Tg(rhs) => {
                self.interpret_node(*rhs);
                self.add_instruction(OpCode::OpTg);
            }
            Node::Ctg(rhs) => {
                self.interpret_node(*rhs);
                self.add_instruction(OpCode::OpCtg);
            }
            Node::Log { value, base } => {
                self.interpret_node(*value);
                self.interpret_node(*base);
                self.add_instruction(OpCode::OpLog);
            }
        }
    }
}

impl Compile for Bytecode {
    type Output = Bytecode;

    fn from_ast(ast: Node) -> Self::Output {
        let mut bytecode = Bytecode::new();
        bytecode.interpret_node(ast);
        bytecode
    }
}
