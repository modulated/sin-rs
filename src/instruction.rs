
#[derive(Debug, PartialEq)]
pub enum Instruction {

    // Basic
    End, // halts execution
    Nop, // no operation
    Set, // push <1> to stack
    Pop, // pop value off stack
    Get, // return top of stack to output register <1>
    Clr, // clear output register <i>
    Ini, // push int from stdin to stack 
    Out, // pop value from stack to stdout

    // Arithmetic
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Control Flow
    Jmp, //Jump to <1> 
    Jmt, //Jump to <1> if top of stack positive
    Jmz, //Jump to <1> if top of stack zero
    Jmn, //Jump to <1> if top of stack negative
}