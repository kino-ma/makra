use crate::ir::Module;

pub struct Compiler {
    module: Module,
}

impl Compiler {
    pub fn new(module: Module) -> Self {
        Self {
            module,
        }
    }
}