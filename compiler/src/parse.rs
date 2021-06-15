use crate::ir::{Module, Section, Function};

pub struct Parser {
    _func: Function
}

impl Parser {
    pub fn new() -> Self {
        let func = Function::new();

        Self {
            _func: func,
        }
    }

    pub fn parse(&self, bytes: &[u8]) -> Result<Module, ()> {
        let bytes_iter = bytes.iter().enumerate();

        // Magic number (4 bytes) and wasm version (4bytes)
        // TODO: ちゃんとパースする
        let mut sections = bytes_iter.skip(4 + 4);

        loop {
            // TODO: push to a vector or something
            match self.parse_section(&mut sections) {
                Ok(_) => (),
                Err(()) => break
            }
        }

        Ok(Module::new())
    }

    pub fn parse_section<I: Iterator<Item = u8>>(&self, sections: &mut I) -> Result<Section, ()> {
        let section_code = sections.next().ok_or(())?;
        let section_size = sections.next().ok_or(())?;

        let section = Section::from_bytes(section_code, section_size, &mut sections);

        Err(())
    }
}

#[cfg(test)]
extern crate std;
mod test {
    extern crate std;

    #[test]
    fn read_wasm() {
        use std::fs;
        use std::string::String;
        use std::io::Read;

        let wasm_binary = {
            let mut f = fs::File::open("wasm-binaries/test.wasm").expect("failed to open wasm: ");
            let mut buf = String::new();
            f.read_to_string(&mut buf);

            buf
        };

        let parser = Parser::new();

        parser.parse(wasm_binary.as_bytes());
    }

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