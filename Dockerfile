FROM easybill/easypwned_bloom_001:v4 AS bloom

FROM ubuntu:24.04
COPY --from=bloom /easypwned.bloom /easypwned.bloom
ADD binary_easypwned /easypwned
ADD binary_easypwned_haveibeenpwned_downloader /easypwned_haveibeenpwned_downloader
ENTRYPOINT ["/easypwned"]