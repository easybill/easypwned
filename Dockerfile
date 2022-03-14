FROM timglabisch/easypwned_bloom_001 AS bloom

FROM ubuntu:20.04
COPY --from=bloom /easypwned.bloom /easypwned.bloom
ADD easypwned /easypwned
ENTRYPOINT ["/easypwned"]