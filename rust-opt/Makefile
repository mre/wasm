all: opt size

build:
	wasm-pack build --release --target web

opt: build
	wasm-opt -Os pkg/hello_world_bg.wasm -o pkg/hello_world_bg_opt.wasm

wat: opt
	wasm2wat pkg/hello_world_bg_opt.wasm

size:
	ls -l pkg/hello_world_bg_opt.wasm

open:
	miniserve .
