# singlestore-wasm-udf-quickjs-eval

A Wasm UDF that evaluate JavaScript using QuickJS, for SingleStore's Wasm Code Engine

### How to build:

Setup Dev Container for SingleStore Wasm Toolkit according to https://singlestore-labs.github.io/singlestore-wasm-toolkit/html/Tutorial-Setup.html

Also need to install `sudo apt-get install -y libclang-dev` in the dev container to avoid `thread 'main' panicked at 'Unable to find libclang: "couldn't find any valid shared libraries matching: ['libclang.so', 'libclang-*.so', 'libclang.so.*', 'libclang-*.so.*'], set the LIBCLANG_PATH environment variable to a path where one of these files can be found ` error when compiling

Set `export QUICKJS_WASM_SYS_WASI_SDK_PATH=/opt/wasi-sdk` 

Build with `cargo wasi build --lib --release`

Deploy with `pushwasm --prompt --wit ./qjs.wit mysql://admin@HOST/DB ./target/wasm32-wasi/release/qjs.wasm qjs_eval`

Then test with `select qjs_eval("udf = function(s) { return 'Hello ' + s }", "World") as result;`, should see `Hello World` as result

### How to use

```
qjs-eval: func(jscode: string, arg1: string) -> string
```

The `jscode` should expose a global function called `udf` that takes a string argument and return a string
