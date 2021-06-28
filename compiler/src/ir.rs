use parity_wasm::elements::Module as WasmModule;

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

    pub fn parse(buf: &[u8]) -> Result<Self, ()> {
        let module = parity_wasm::deserialize_buffer(buf)
            .or(Err(()))?;
        Ok(Self::new(module))
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::prelude::v1::*;

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