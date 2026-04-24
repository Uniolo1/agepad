all:
	cargo run
build:
	cargo build --release
test:
	cargo test -- --test-threads=1
