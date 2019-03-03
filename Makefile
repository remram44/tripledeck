.PHONY: all program wasm docker-serve

all: program wasm

program: program/target/debug/tripledeck

program/target/debug/tripledeck: program/Cargo.toml $(wildcard program/src/*.rs)
	cd program && cargo build

wasm: webapp/dist/tripledeck_wasm.js

webapp/dist/tripledeck_wasm.js: webapp/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm $(wildcard webapp/*.html) $(wildcard webapp/*.js)
	mkdir -p webapp/dist
	cd webapp && wasm-bindgen target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm --out-dir ../webapp/dist/

webapp/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm: webapp/Cargo.toml $(wildcard webapp/src/*.rs)
	cd webapp && cargo build --target wasm32-unknown-unknown

test:
	cd core && cargo test
	cd program && cargo test
	cd webapp && cargo test

docker-serve: wasm
	cd webapp && ./docker.sh serve
