build:
	RUSTFLAGS='-C opt-level=s' wasm-pack build -s squirrelchat --release client
	RUSTFLAGS='-C opt-level=3' wasm-pack build -s squirrelchat --release --target nodejs server

format:
	rustfmt client/**/*.rs core/**/*.rs server/**/*.rs

lint:
	cargo deny check
	cargo clippy
	rustfmt --check client/**/*.rs core/**/*.rs server/**/*.rs
