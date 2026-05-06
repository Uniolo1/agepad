all:
	cargo run
build:
	cargo build --release
test:
	cargo test
testget:
	cargo run -- -ga
commit:
	# helper
	git commit -s
	cargo test # run 'cargo test' afterwards to ensure they are aware of testing status
