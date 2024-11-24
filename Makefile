.PHONY: build test

build:
	echo "Building Windows Release" && cross build --release --target x86_64-pc-windows-gnu && echo "Windows Release Build Success"
	echo "Building Linux Release" && cross build --release --target x86_64-unknown-linux-musl && echo "Linux Release Build Success"

test:
	echo "Testing"

run:
	cargo run
