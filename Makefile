.PHONY: all program wasm

all: program wasm

program: program/target/debug/tripledeck

program/target/debug/tripledeck: program/Cargo.toml $(wildcard program/src/*.rs)
	cd program && cargo build

wasm: webapp/dist/tripledeck_wasm.js

webapp/dist/tripledeck_wasm.js: wasm/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm $(wildcard webapp/*.html) $(wildcard webapp/*.js)
	mkdir -p webapp/dist
	cd wasm && wasm-bindgen target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm --out-dir ../webapp/dist/

wasm/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm: wasm/Cargo.toml $(wildcard wasm/src/*.rs)
	cd wasm && cargo build --target wasm32-unknown-unknown
