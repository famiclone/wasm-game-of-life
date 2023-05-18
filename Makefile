build:
	rustc -C opt-level=s --target wasm32-unknown-unknown game.rs
