pub mod token;
pub use token::Token;
pub mod instruction;
use instruction::Instruction;
pub mod error;
use error::*;

pub type Byte = u8;

/// Takes a slice of strings and outputs tokens 
pub fn parse_strings_to_tokens(tokens: &[&str]) -> Result<Vec<Token>, Error> {
    
    let mut res = Vec::with_capacity(tokens.len());

    if !(*tokens).contains(&"end") {
        return Err(Error::new(ErrorKind::NoEndInstruction, "Not found".to_owned()));
    }

    for t in tokens {
        if let Ok(x) = t.parse::<i64>() {
            res.push(Token::Num(x));
            continue;      
        }
        match *t {
            "nop" => res.push(Token::Instr(Instruction::Nop)),
            "end" => res.push(Token::Instr(Instruction::End)),
            "set" => res.push(Token::Instr(Instruction::Set)),
            
            

            e => return Err(Error::new(ErrorKind::UnknownInstruction, e.to_owned())),
        };
    }

    Ok(res)
}

#[test]
fn test_parse_strings_to_tokens() {
    let tokens = vec![Token::Instr(Instruction::Nop), Token::Instr(Instruction::Set), Token::Num(51), Token::Instr(Instruction::End)];
    let strings = vec!["nop", "set", "51", "end"];
    let out = parse_strings_to_tokens(&strings).unwrap();

    assert_eq!(tokens, out);
}

#[test]
fn test_parse_strings_to_tokens_no_end() {
   
    let out = parse_strings_to_tokens(&["nop"]);

    assert_eq!(ErrorKind::NoEndInstruction, out.unwrap_err().kind);
}

#[test]
fn test_parse_strings_to_tokens_unknown_instruction() {
   
    let out = parse_strings_to_tokens(&["foo", "end"]);

    assert_eq!(ErrorKind::UnknownInstruction, out.unwrap_err().kind);
}