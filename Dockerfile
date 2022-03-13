#FROM clux/muslrust:latest AS compiler
#ADD src /tmp/proj/src
#ADD Cargo.toml /tmp/proj/Cargo.toml
#ADD Cargo.lock /tmp/proj/Cargo.lock
#RUN cd /tmp/proj && cargo build --target=x86_64-unknown-linux-musl --release
#RUN cd /tmp/proj && rustup target add aarch64-unknown-linux-musl && cargo build --target=aarch64-unknown-linux-musl --release

FROM ubuntu:20.04
#COPY --from=compiler /tmp/proj/target/aarch64-unknown-linux-musl/release/easypwned /easypwned
ADD easypwned_$TARGETPLATFORM /easypwned
RUN ls