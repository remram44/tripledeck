.PHONY: all core program wasm webapp serve

all: program wasm

core: core/Cargo.toml $(wildcard core/src/*.rs)

program: program/target/debug/tripledeck

program/target/debug/tripledeck: core program/Cargo.toml $(wildcard program/src/*.rs)
	cd program && cargo build

wasm: webapp/dist/tripledeck_wasm.js

webapp/dist/tripledeck_wasm.js: webapp/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm $(wildcard webapp/*.html) $(wildcard webapp/*.js)
	mkdir -p webapp/dist
	cd webapp && wasm-bindgen target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm --out-dir ../webapp/dist/

webapp/target/wasm32-unknown-unknown/debug/tripledeck_wasm.wasm: core webapp/Cargo.toml $(wildcard webapp/src/*.rs)
	cd webapp && cargo build --target wasm32-unknown-unknown

test:
	cd core && cargo test
	cd program && cargo test
	cd webapp && cargo test

webapp: wasm
ifdef docker
	cd webapp && ./docker.sh build
else
	cd webapp && node_modules/webpack/bin/webpack.js
endif

serve: wasm
ifdef docker
	cd webapp && ./docker.sh serve
else
	cd webapp && npm run serve
endif
