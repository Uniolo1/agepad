all:
	cargo run
build:
	cargo build --release
test:
	cargo test -- --test-threads=1
testget:
	cargo run -- -ga
