(module
  (func (result i32) (local i32 i32 i32 i32)
    ;; int x = 10
    i32.const 10
    local.set 0
    ;; int y = 20
    i32.const 20
    local.set 1
    ;; int sum = 0
    i32.const 0
    local.set 2
    ;; int counter = 5
    i32.const 5
    local.set 3

    ;; for (int counter = 5; i > 0; i -= 1)
    loop
      local.get 0
      local.get 1
      i32.add
      local.get 2
      i32.add
      local.set 2

      ;; counter -= 1
      local.get 3
      i32.const 1
      i32.sub
      local.set 3
      local.get 3
      ;; if counter == 0 then break
      i32.const 0
      i32.ne
      br_if 0
    end

    local.get 2
    ))
