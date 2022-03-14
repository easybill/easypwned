build_bloom:
	curl https://downloads.pwnedpasswords.com/passwords/pwned-passwords-sha1-ordered-by-hash-v8.7z --output pwned-passwords-sha1-ordered-by-hash-v8.7z
	7z e pwned-passwords-sha1-ordered-by-hash-v8.7z
	rm pwned-passwords-sha1-ordered-by-hash-v8.7z
	cargo run --release -- --create_bloom_file_from_file=pwned-passwords-sha1-ordered-by-hash-v8.txt
	rm pwned-passwords-sha1-ordered-by-hash-v8.txt

build-easypwned_bloom_001:
	cp easypwned.bloom .docker/easypwned_bloom_001/easypwned.bloom
	cd .docker/easypwned_bloom_001 && docker build -t timglabisch/easypwned_bloom_001:latest .
	rm .docker/easypwned_bloom_001/easypwned.bloom
	docker push timglabisch/easypwned_bloom_001:latest