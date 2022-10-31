use wasmedge_sdk::{params, Executor, Module, Store, WasmVal};
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

    // loads a wasm module from the given in-memory bytes
    let module = Module::from_bytes(None, &wasm_bytes)?;

    // create an executor
    let mut executor = Executor::new(None, None)?;

    // create a store
    let mut store = Store::new()?;

    // register the module into the store
    let extern_instance = store.register_named_module(&mut executor, "extern", &module)?;

    // get the exported function "fib"
    let run = extern_instance.func("fib").ok_or(anyhow::Error::msg(
        "Not found exported function named 'fib'.",
    ))?;

    // runs the exported WASM function
    let returns = run.call(&mut executor, params!(10))?;

    println!("{}", returns[0].to_i32());
    
    Ok(())
}

