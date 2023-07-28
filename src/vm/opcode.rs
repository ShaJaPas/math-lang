use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum OpCode {
    OpPush(f32),
    OpPop,
    OpAdd,
    OpSubstract,
    OpMultiply,
    OpDivide,
    OpPower,
    OpMod,
    OpDiv,
    OpUnaryMinus,
    OpFactorial,
    OpSin,
    OpCos,
    OpTg,
    OpCtg,
    OpLog,
}
