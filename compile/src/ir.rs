use alloc::prelude::v1::*;

use parity_wasm::elements::{Module as WasmModule, FuncBody};

use crate::err::{Result, Error::Failure};

/// Intermidate representation of a WebAssembly Module
#[cfg_attr(test, derive(Debug))]
pub struct Module {
    inner: WasmModule,
}

impl Module {
    pub fn new(module: WasmModule) -> Self {
        Self {
            inner: module,
        }
    }

    pub fn parse(buf: &[u8]) -> Result<Self> {
        let module = parity_wasm::deserialize_buffer(buf)
            .or(Err(Failure))?;
        Ok(Self::new(module))
    }

    pub fn generate(&self) -> Result<Vec<u8>> {
        let bodies = self.inner.code_section().ok_or(Failure)?.bodies();
        let v = bodies.iter().map(generate_func).try_fold(Vec::new(), |mut v, bin| {
            v.append(&mut bin?);
            Ok(v)
        })?;
        Ok(v)
    }
}

fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    Err(Failure)
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

        println!("module: {:#?}", module);

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