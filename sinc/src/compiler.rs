use sin_rs::{Byte, Token};

#[allow(dead_code)]
pub fn compile(tokens: &[&str]) -> Vec<Byte> {
    let out = Vec::with_capacity(tokens.len());

    out
}

pub fn write_bytecode(filename: &String, bytecode: &Vec<Byte>) -> Result<(), std::io::Error> {
    std::fs::write(filename, bytecode)?;
    Ok(())
}