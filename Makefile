debug:
	cargo build --target=wasm32-unknown-emscripten
	mkdir -p ./html
	cp ./target/wasm32-unknown-emscripten/debug/deps/*.data ./html/
	cp ./target/wasm32-unknown-emscripten/debug/*.wasm ./html/
	cp ./target/wasm32-unknown-emscripten/debug/*.d ./html/
	cp ./target/wasm32-unknown-emscripten/debug/*.js ./html/
