# SPDX-FileCopyrightText: 2026 uniolo1
# SPDX-License-Identifier: GPL-3.0-only

all:
	cargo run
build:
	cargo build --release
test:
	cargo test
testget:
	cargo run -- -ga
