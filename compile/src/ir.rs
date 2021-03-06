use alloc::prelude::v1::*;

use parity_wasm::elements::Module as WasmModule;

use crate::codegen::Generator;
use crate::err::{
    Error::{Failure, ParseFailure},
    Result,
};

/// Intermidate representation of a WebAssembly Module
#[cfg_attr(test, derive(Debug))]
pub struct Module {
    inner: WasmModule,
}

impl Module {
    pub fn new(module: WasmModule) -> Self {
        Self { inner: module }
    }

    pub fn parse(buf: &[u8]) -> Result<Self> {
        let module = parity_wasm::deserialize_buffer(buf).map_err(ParseFailure)?;
        Ok(Self::new(module))
    }

    pub fn generate(&self) -> Result<Vec<u8>> {
        let bodies = self.inner.code_section().ok_or(Failure)?.bodies();
        let v = bodies
            .iter()
            .map(|body| Generator::new(body).generate())
            .try_fold(Vec::new(), |mut v, bin| {
                v.append(&mut bin?);
                Ok(v)
            })?;
        Ok(v)
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::prelude::v1::*;
    use std::println;

    use super::*;

    #[test]
    fn parse_module() {
        let wasm_binary = get_wasm_binary();
        let module = Module::parse(&wasm_binary[..]);

        module.expect("failed to parse module");
    }

    fn get_wasm_binary() -> Vec<u8> {
        use std::fs;
        use std::io::Read;

        let mut f = fs::File::open("wasm-binaries/test.wasm").expect("failed to open wasm: ");
        let mut buf = Vec::new();
        if f.read_to_end(&mut buf).expect("fail reading") == 0 {
            panic!("not enaugh content")
        };

        buf
    }
}
