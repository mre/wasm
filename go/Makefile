all: build templates open

.PHONY: build
build:
	GOOS=js GOARCH=wasm go build -o test.wasm main.go

# Yes, Go ships JavaScript and HTML template files for WASM 
# with the distribution
.PHONY: templates
templates:
	cp "$(shell go env GOROOT)/misc/wasm/wasm_exec.js" .
	cp "$(shell go env GOROOT)/misc/wasm/wasm_exec.html" .

.PHONY: open
open:
	echo "Starting miniserve (https://github.com/svenstaro/miniserve)"
	miniserve .

.PHONY: optimize
optimize:
	wasm-opt -Oz -o test.wasm test.wasm
