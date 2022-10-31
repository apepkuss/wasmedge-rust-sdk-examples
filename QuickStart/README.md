# Quick Start

Getting started here, we'll help you quickly get familiar with the [WasmEdge Rust SDK](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/index.html). Through two examples, we hope that you can have an overall concept about WasmEdge.

The first example uses [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html), which defines a virtual environment for managing WASM programs and encapsulates many of the internal details.

In the second example, we no longer use the [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html), but run the Wasm Module through some fine-grained API. Through the second, you can not only meet the counterparts of a series of concepts defined by [WASM Specifications](https://webassembly.github.io/spec/) in WasmEdge, but also understand how to control your program finely, that is to say, it gives you more freedom without using [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html).

In both examples, we use in-memory wasm bytes, which define the [Fibonacci function](https://en.wikipedia.org/wiki/Fibonacci_number). It doesn't matter whether you are familiar with the relevant syntax of [wat](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format), just know that the following wasm bytes exports a function called `fib`, which takes an input of type `i32` and outputs the corresponding Fibonacci sequence value.

```rust
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
```

## With `Vm`

As mentioned before, in the first example we used [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) which encapsulates a lot of the implementation. We may just need one line of code to run the `fib` function above.

```rust
// create a Vm context and invokes an exported function from the given in-memory wasm bytes
let returns = Vm::new(None)?.run_func_from_bytes(&wasm_bytes, "fib", params!(10))?;
```

How simple it is! Of course, the APIs provided by [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) are much more than that. For example, we can achieve the same goal in the following way.

```rust
// create a Vm context
let mut vm = Vm::new(None)?;

// register a wasm module from the given in-memory wasm bytes and instantiates it
vm = vm.register_module_from_bytes("extern", wasm_bytes)?;

// runs an exported WASM function
let returns = vm.run_func(Some("extern"), "fib", params!(10))?;
```

After creating a [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html) context, use the API [`register_module_from_bytes`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html#method.register_module_from_bytes) to register the previous wasm bytes (and then instantiate it). Finally, the function `fib` can be run by calling [`run_func`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html#method.run_func).

Furthermore, we can invoke an exported function from the given WASM file or a compiled WASM [`Module`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Module.html). We can also check which [`Modules`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Module.html) are registered in the [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html). [Here](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html), there may be the APIs you want.

## Without `Vm`

Besides the WASM execution through the [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html), we can execute the WASM functions or instantiate WASM modules step-by-step with the [`Module`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Module.html), [`Executor`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Executor.html), and [`Store`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Store.html) contexts (If you know WASM, you've probably heard of these concepts). Maybe this way of use gives you more freedom, allowing you to use WasmEdge as you want without being restricted by [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html). Experience has shown that it makes more sense to integrate WasmEdge into your project in this way. Let's explore the related concepts with the following example, which has the same result as the example above.

```rust
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
```

First, we create a [`Module`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Module.html) context, which defines compiled in-memory representation of an input WASM bytes. Then, we create a [`Executor`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Executor.html) context, which defines an execution environment for both pure WASM and compiled WASM, and create a [`Store`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Store.html) context, which represents all global state that can be manipulated by WASM programs. Next, is the process of instantiation, by calling [`register_named_module`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Store.html#method.register_named_module), we can achieve this goal. After that, call the method [`func`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Instance.html#method.func) of [`Instance`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Instance.html), we can get the exported function instance called `fib`. Finally, call the [`call`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Func.html#method.call) method of [`Func`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Func.html#) to run the Fibonacci function. Although tedious, does this step-by-step approach evoke your memory of WASM-related concepts?

[`Module`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Module.html), [`Executor`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Executor.html), and [`Store`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Store.html) contexts provide a variety of APIs. By using these APIs in combination, we can implement any functionality provided by the [`Vm`](https://wasmedge.github.io/WasmEdge/wasmedge_sdk/struct.Vm.html), but with more powerful magic (Maybe it's not obvious now, but in the following chapters, you will surely find out).

