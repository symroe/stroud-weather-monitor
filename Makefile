
cargo-lambda:
	wget https://github.com/cargo-lambda/cargo-lambda/releases/download/v0.17.2/cargo-lambda-v0.17.2.x86_64-unknown-linux-musl.tar.gz
	tar -zvxf cargo-lambda-v0.17.2.x86_64-unknown-linux-musl.tar.gz
	rm cargo-lambda-v0.17.2.x86_64-unknown-linux-musl.tar.gz

/home/circleci/.cargo/bin/zig:
	wget https://ziglang.org/download/0.10.1/zig-linux-x86_64-0.10.1.tar.xz
	tar -xf zig-linux-x86_64-0.10.1.tar.xz
	cp -r zig-linux-x86_64-0.10.1/* /home/circleci/.cargo/bin/
	rm zig-linux-x86_64-0.10.1.tar.xz
	rm -rf zig-linux-x86_64-0.10.1

ci-install-deps: cargo-lambda /home/circleci/.cargo/bin/zig

.PHONY: build
build:
	./cargo-lambda lambda build --release
	rm -rf ./build
	mkdir -p ./build
	cp -v ./target/lambda/lora-monitor/bootstrap ./build/bootstrap
