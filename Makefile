
.PHONY: build
build:
	cargo lambda build --release
	rm -rf ./build
	mkdir -p ./build
	cp -v ./target/lambda/lora-monitor/bootstrap ./build/bootstrap
