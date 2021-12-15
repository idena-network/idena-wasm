cargo build --release --target=x86_64-pc-windows-gnu --verbose
cp .\target\x86_64-pc-windows-gnu\release\idena_wasm.dll ..\go-binding\lib\idena_wasm.dll
cbindgen --config cbindgen.toml --crate idena-wasm --output bindings.h
cp .\bindings.h ..\go-binding\lib\bindings.h
