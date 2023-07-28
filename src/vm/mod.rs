pub mod bytecode;
pub mod opcode;

use bincode::config;

use bytecode::Bytecode;

use opcode::OpCode;

use crate::operations::Op;

pub struct VM {
    bytecode: Bytecode,
    stack: Vec<f32>,
    ip: usize,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        let config = config::standard();
        Self {
            bytecode: bincode::decode_from_slice(&bytecode, config).unwrap().0,
            stack: Vec::with_capacity(512),
            ip: 0,
        }
    }
}

pub trait StackVM {
    type Stack;

    fn exec_next(&mut self) -> Result<bool, String>;
    fn pop(&mut self) -> Self::Stack;
    fn push(&mut self, node: Self::Stack);
}

impl StackVM for VM {
    type Stack = f32;

    fn exec_next(&mut self) -> Result<bool, String> {
        if self.ip >= self.bytecode.instructions.len() {
            return Ok(false);
        }
        let instruction = &self.bytecode.instructions[self.ip];
        match instruction {
            OpCode::OpPush(value) => self.push(*value),
            OpCode::OpPop => unimplemented!(),
            OpCode::OpAdd => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::add(lhs, rhs));
            }
            OpCode::OpSubstract => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::substract(lhs, rhs));
            }
            OpCode::OpMultiply => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::multiply(lhs, rhs));
            }
            OpCode::OpDivide => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::divide(lhs, rhs)?);
            }
            OpCode::OpPower => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::power(lhs, rhs));
            }
            OpCode::OpMod => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::modulus(lhs, rhs)?);
            }
            OpCode::OpDiv => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(Op::div(lhs, rhs)?);
            }
            OpCode::OpUnaryMinus => {
                let rhs = self.pop();
                self.push(Op::unary_minus(rhs));
            }
            OpCode::OpFactorial => {
                let rhs = self.pop();
                self.push(Op::factorial(rhs)?);
            }
            OpCode::OpSin => {
                let rhs = self.pop();
                self.push(Op::sin(rhs));
            }
            OpCode::OpCos => {
                let rhs = self.pop();
                self.push(Op::cos(rhs));
            }
            OpCode::OpTg => {
                let rhs = self.pop();
                self.push(Op::tg(rhs));
            }
            OpCode::OpCtg => {
                let rhs = self.pop();
                self.push(Op::ctg(rhs));
            }
            OpCode::OpLog => {
                let lhs = self.pop();
                let rhs = self.pop();
                self.push(Op::log(rhs, lhs)?);
            }
        }

        self.ip += 1;
        Ok(true)
    }

    fn pop(&mut self) -> Self::Stack {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, node: Self::Stack) {
        self.stack.push(node)
    }
}
