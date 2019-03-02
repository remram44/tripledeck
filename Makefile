.PHONY: all program wasm

all: program wasm

program: program/target/debug/tripledeck

program/target/debug/tripledeck:
	cd program && cargo build

wasm: webapp/dist/tripledeck_wasm.js

webapp/dist/tripledeck_wasm.js: wasm/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm
	mkdir -p webapp/dist
	cd wasm && wasm-bindgen target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm --out-dir ../webapp/dist/

wasm/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm:
	cd wasm && cargo build --target wasm32-unknown-unknown
