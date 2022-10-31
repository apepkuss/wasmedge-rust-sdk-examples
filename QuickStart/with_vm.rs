use wasmedge_sdk::{params, Vm, WasmVal};
use wasmedge_types::wat2wasm;

#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = wat2wasm(
        br#"(module
        (export "fib" (func $fib))
        (func $fib (param $n i32) (result i32)
         (if
          (i32.lt_s
           (get_local $n)
           (i32.const 2)
          )
          (return
           (i32.const 1)
          )
         )
         (return
          (i32.add
           (call $fib
            (i32.sub
             (get_local $n)
             (i32.const 2)
            )
           )
           (call $fib
            (i32.sub
             (get_local $n)
             (i32.const 1)
            )
           )
          )
         )
        )
       )
    "#,
    )?;

    // create a Vm context and invokes an exported function from the given in-memory wasm bytes
    let returns = Vm::new(None)?.run_func_from_bytes(&wasm_bytes, "fib", params!(10))?;
    println!("{}", returns[0].to_i32());

    Ok(())
}

