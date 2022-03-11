download:
	wget https://downloads.pwnedpasswords.com/passwords/pwned-passwords-sha1-ordered-by-hash-v8.7z

build-linux-cross:
	cross build -vv --target x86_64-unknown-linux-musl
	cross build --target aarch64-unknown-linux-musl

build-linux:
	docker run --platform linux/amd64 -v "$(CURDIR)":/volume -w /volume -e RUSTFLAGS='-C link-args=-s' -t clux/muslrust cargo build --target=x86_64-unknown-linux-musl --release
	cp target/release/easypwned dist/easypwned_osx_x86_64
	cp target/x86_64-unknown-linux-musl/release/easypwned dist/easypwned_linux_x86_64
	cp easypwned.bloom dist/easypwned.bloom

build-easypwned_bloom_001:
	cp easypwned.bloom .docker/easypwned_bloom_001/easypwned.bloom
	cd .docker/easypwned_bloom_001 && docker build -t timglabisch/easypwned_bloom_001:latest .
	rm .docker/easypwned_bloom_001/easypwned.bloom
	docker push timglabisch/easypwned_bloom_001:latest