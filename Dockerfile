FROM easybill/easypwned_bloom_001:v2 AS bloom

FROM ubuntu:23.04
COPY --from=bloom /easypwned.bloom /easypwned.bloom
ADD easypwned /easypwned
ADD easypwned_haveibeenpwned_downloader /easypwned_haveibeenpwned_downloader
ENTRYPOINT ["/easypwned"]