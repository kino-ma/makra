use crate::ir::{Module, Section, Function};
use num_traits::{Unsigned, NumCast};

use nom::{IResult};
use nom::error::{ParseError, ContextError};
use nom::bytes::streaming;
use nom::sequence::{tuple};
use nom::multi::{fold_many1};
use nom_leb128::{leb128_i32, leb128_u32, leb128_i64};

pub fn parser<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], (), E> 
{
    Ok((i, ()))
}

fn module<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], &'a [u8], E> {
    // first 4 bytes are wasm's magic number "\0asm"
    let magic_number = streaming::tag(b"\0asm");
    // and following 4bytes are wasm version
    let wasm_version = streaming::tag(&[0x01, 0x00, 0x00, 0x00]);

    let sections = fold_many1(section);

    Ok((i, i))
}

fn section<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
  i: &'a [u8],
) -> IResult<&'a [u8], &'a [u8], E> {
    TODO: implement parser
    let code = streaming::take(1usize)
    let size = leb128_u32();
    TODO: add global allocator, and use many_SOMETHING
    let content = 

    //tuple((code, size, content));

    Ok((i, i))
}

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

pub fn decode_uleb128<N: Unsigned + NumCast, I: Iterator<Item = u8>>(bytes: &mut I) -> Result<N, ()> {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let byte = bytes.next().ok_or(())?;
        // Rust alerts normal `<<` operator on overflow
        result |= (byte & 0b0111_1111).checked_shl(shift).unwrap_or(0);
        if byte & 0b1000_0000 == 0 {
            break;
        }
        shift += 7;
    }

    let res: N = num_traits::cast::cast(result).ok_or(())?;
    return Ok(res);
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
        use super::Parser;

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
        use super::Parser;
        use crate::ir::IR;

        let function_body = [0x41, 0x0a, 0x41, 0x14, 0x6a];
        let parser = Parser::new();

        let res = parser.parse(&function_body).expect("failed to parse");
        let expected = {
            let ir1 = IR::new(&function_body[0..2]);
            let ir2 = IR::new(&function_body[2..5]);
            let ir3 = IR::new(&function_body[..]);
        };
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