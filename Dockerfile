FROM clux/muslrust:latest AS compiler
ADD src /tmp/proj/src
ADD Cargo.toml /tmp/proj/Cargo.toml
ADD Cargo.lock /tmp/proj/Cargo.lock
RUN cd /tmp/proj && cargo build --target=x86_64-unknown-linux-musl --release

#FROM ubuntu:22.04 AS downloader
#RUN apt -y install p7zip-full
#RUN wget https://downloads.pwnedpasswords.com/passwords/pwned-passwords-sha1-ordered-by-hash-v8.7z
#RUN 7z x pwned-passwords-sha1-ordered-by-hash-v8.7z
#RUN rm pwned-passwords-sha1-ordered-by-hash-v8.7z
#RUN mv pwned-passwords-sha1-ordered-by-hash-v8.txt /pwned-passwords-sha1-ordered-by-hash-v8.txt


FROM ubuntu:20.04
COPY --from=compiler /tmp/proj/target/x86_64-unknown-linux-musl/release/easypwned /easypwned
RUN ls