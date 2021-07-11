(module
  (func (result i32) (local i32 i32)
    i32.const 10
    local.set 0
    i32.const 20
    local.set 1
    local.get 0
    local.get 1
    i32.add))