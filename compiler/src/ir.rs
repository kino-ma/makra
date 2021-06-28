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

    pub fn parse_buffer(buf: &[u8]) -> Result<Self, ()> {
        let module = parity_wasm::deserialize_buffer(buf)
            .or(Err(()))?;
        Ok(Self::new(module))
    }
}

#[cfg(test)]
mod test {
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}