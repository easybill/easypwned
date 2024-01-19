build_bloom:
	cargo run --release --bin easypwned_haveibeenpwned_downloader -- --sink-bloom-file=easypwned.bloom

run_release:
	cargo run --release --bin easypwned

build-easypwned_bloom:
	cp easypwned.bloom .docker/easypwned_bloom_001/easypwned.bloom
	cd .docker/easypwned_bloom_001 && docker build -t easybill/easypwned_bloom_001:v3 .
	rm .docker/easypwned_bloom_001/easypwned.bloom
	docker push easybill/easypwned_bloom_001:v3