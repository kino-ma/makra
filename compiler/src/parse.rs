use crate::ir::{IR, Function, IRType::*};

pub struct Parser {
    func: Function
}

impl Parser {
    pub fn new() -> Self {
        let func = Function::new();

        Self {
            func,
        }
    }

    pub fn parse(self, function_body: &[u8]) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
extern crate std;
mod test {
    use super::*;

    #[test]
    fn add_two_numbers() {
        let function_body = [0x41, 0x0a, 0x41, 0x14, 0x6a];
        let parser = Parser::new();

        let res = parser.parse(&function_body).expect("failed to parse");
        let expected = {
            let it1 = IR::new(&function_body[0..2]);
            let it1 = IR::new(&function_body[2..5]);
            let it1 = IR::new(&function_body[..]);
        };
    }
}