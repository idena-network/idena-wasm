cargo build --release --target=x86_64-pc-windows-gnu --verbose
cp .\target\x86_64-pc-windows-gnu\release\libidena_wasm.a ..\idena-wasm-binding\lib\libidena_wasm_windows_amd64.a
cbindgen --config cbindgen.toml --crate idena-wasm --output bindings.h
cp .\bindings.h ..\idena-wasm-binding\lib\bindings.h
