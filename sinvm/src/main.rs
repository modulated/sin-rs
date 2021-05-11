mod vm;
use vm::VM;

fn main() {
    println!("SIN VM");
    let vm = VM::new(&[0xfa, 0x00, 0xac]);
}
