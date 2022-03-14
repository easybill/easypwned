FROM clux/muslrust:latest AS bloom

FROM ubuntu:20.04
COPY --from=bloom /easypwned.bloom /easypwned.bloom
ADD easypwned /easypwned