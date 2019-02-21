.PHONY: all program wasm

all: program wasm

program: program/target/debug/tripledeck

program/target/debug/tripledeck:
	cd program && cargo build

wasm: webapp/tripledeck_wasm.js

webapp/tripledeck_wasm.js: wasm/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm
	cd wasm && wasm-bindgen target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm --out-dir ../webapp

wasm/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm:
	cd wasm && cargo build --target wasm32-unknown-unknown
