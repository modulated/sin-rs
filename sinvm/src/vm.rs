

struct VM<T> {
    stack: Vec<T>,
    instructions: Vec<String>,
    ip: usize,    
}

impl<T> VM<T> {
    pub fn new(bytecode: &[&str]) -> VM<T> {
        let mut string: Vec<String> = Vec::with_capacity(bytecode.len());
        for s in bytecode {
            string.push(s.to_string());
        }

        VM {
            stack: Vec::new(),
            instructions: string,
            ip: 0
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn run(&self) {
        println!("Running");
    }
}