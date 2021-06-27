use num_traits::{Unsigned, NumCast};

use nom::{IResult};
use nom::error::{ParseError, ContextError, ErrorKind};
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

    map(tuple((magic_number, wasm_version, sections)), Module::new)(i)
}

fn section<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], Section, E> {
    let code = onebyte;
    let size = leb128_u32;
    let content = multi::many1(onebyte);

    map(
        tuple((code, size, content)),
        Section::new,
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
    
    use nom::error::Error;

    use super::*;

    #[test]
    fn parse_module() {
        use std::fs;
        use std::string::String;
        use std::io::Read;

        use nom::combinator::cut;

        let wasm_binary = {
            let mut f = fs::File::open("wasm-binaries/test.wasm").expect("failed to open wasm: ");
            let mut buf = String::new();
            f.read_to_string(&mut buf);

            buf
        };

        cut::<_, _, Error<_>, _>(module)(wasm_binary.as_bytes()).unwrap();
    }

    /*#[test]
    fn should_decode_uleb128() {
        use super::decode_uleb128;

        let bytes = [0xE5 as u8, 0x8E, 0x86];
        let expected = 624485;

        let result: u32 = decode_uleb128(&mut bytes.iter().cloned()).expect("couldn't decode");

        assert_eq!(result, expected);
    }*/
}