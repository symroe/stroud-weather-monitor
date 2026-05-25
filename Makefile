
cargo-lambda:
	wget https://github.com/cargo-lambda/cargo-lambda/releases/download/v0.17.2/cargo-lambda-v0.17.2.x86_64-unknown-linux-musl.tar.gz
	tar -zvxf cargo-lambda-v0.17.2.x86_64-unknown-linux-musl.tar.gz
	rm cargo-lambda-v0.17.2.x86_64-unknown-linux-musl.tar.gz

ci-install-deps: cargo-lambda

.PHONY: build
build:
	./cargo-lambda lambda build --release --compiler cargo
	rm -rf ./build
	mkdir -p ./build
	cp -v ./target/lambda/lora-monitor/bootstrap ./build/bootstrap
