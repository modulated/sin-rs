use super::{Instruction};

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(i64),
    Var((String, String)),
    Instr(Instruction)
}