cargo build --release --target=x86_64-pc-windows-gnu --verbose
cp .\target\x86_64-pc-windows-gnu\release\libidena_wasm.a ..\go-binding\lib\libidena_wasm.a
cbindgen --config cbindgen.toml --crate idena-wasm --output bindings.h
cp .\bindings.h ..\go-binding\lib\bindings.h
