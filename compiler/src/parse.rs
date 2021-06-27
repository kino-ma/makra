use num_traits::{Unsigned, NumCast};

use nom::{IResult};
use nom::error::{context, ParseError, ContextError};
use nom::bytes::streaming;
use nom::number::streaming::u8 as onebyte;
use nom::sequence::{tuple};
use nom::combinator::{map};
use nom::multi;
use nom_leb128::{leb128_i32, leb128_u32, leb128_i64};

use crate::ir::{Module, Section, Function};

pub fn parser<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], (), E> 
{
    Ok((i, ()))
}

fn module<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], Module, E> {
    // first 4 bytes are wasm's magic number "\0asm"
    let magic_number = streaming::tag(b"\0asm");
    // and following 4bytes are wasm version
    let wasm_version = leb128_u32;

    let sections = multi::many1(section);

    context(
        "module",
        map(tuple((magic_number, wasm_version, sections)), Module::new)
    )(i)
}

fn section<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], Section, E> {
    let code = onebyte;
    let size = leb128_u32;
    let content = multi::many1(onebyte);

    context(
        "section",
        map(
            tuple((code, size, content)),
            Section::new,
        ),
    )(i)
}

#[cfg(hoge)]
pub struct Parser {
    _func: Function
}

#[cfg(hoge)]
impl Parser {
    pub fn new() -> Self {
        let func = Function::new();

        Self {
            _func: func,
        }
    }

    pub fn parse(&self, bytes: &[u8]) -> Result<Module, ()> {
        let bytes_iter = bytes.iter();

        // Magic number (4 bytes) and wasm version (4bytes)
        // TODO: ちゃんとパースする
        let mut sections = bytes_iter.skip(4 + 4).cloned();

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
        let section = Section::from_bytes(sections);
        Err(())
    }
}

#[cfg(test)]
extern crate std;
mod test {
    extern crate std;

    use nom::error::{VerboseError, convert_error};

    use super::*;

    #[test]
    fn parse_module() {
        use std::fs;
        use std::string::String;
        use std::io::Read;
        use std::vec::Vec;

        let wasm_binary = {
            let mut f = fs::File::open("wasm-binaries/test.wasm").expect("failed to open wasm: ");
            let mut buf = Vec::new();
            if f.read_to_end(&mut buf).expect("fail reading") == 0 {
                panic!("not enaugh content")
            };

            buf
        };
        let parsed_module = module::<'_, VerboseError<&[u8]>>(&wasm_binary[..]);

        parsed_module.expect("failed to parse module");
    }
}