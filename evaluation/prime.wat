(module
  (func (result i32) (local i32 i32)
    ;; int n = 2147483647
    i32.const 2147483647
    local.set 0

    ;; int n = 1
    i32.const 1
    local.set 1

    ;; loop {}
    loop
      ;; i += 1
      i32.const 1
      local.get 1
      i32.add
      local.set 1

      ;; if n % i == 0 then break
      ;; n % i == 0
      local.get 0
      local.get 1
      i32.rem_u
      i32.const 0
      i32.ne
      
      ;; then break
      br_if 0
    end

    local.get 0
    local.get 1
    i32.sub
    ))
