run:
	cargo run

run-fast:
	cargo run --features bevy/dynamic_linking

release:
	cargo build --release