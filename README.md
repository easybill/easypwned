# easypwned (haveibeenpwned / HIBP)
Rest API to check if a password is in a data breach. Works offline - everything stays on your machine! Database is included.
We also provide a [downloader for the hibp database](https://github.com/easybill/easypwned#download-the-haveibeenpwned--hibp-database-pwnedpasswordsdownloader).

## Example
The simplest way to run it is using docker:

```bash
docker run --rm --network=host easybill/easypwned:v0.0.26
curl http://127.0.0.1:3342/pw/[BLANK_PASSWORD]  # use /hash/SHA1 in prod apps (pw/[PW] is for testing).
curl http://127.0.0.1:3342/hash/0000001C5F765AA063E4F8470451F85F7DB4ED3A # << UPPERCASE(SHA1(PLAINTEXT))
```
The [dockerimage](https://hub.docker.com/r/easybill/easypwned) comes with a list of compromised passwords in the form of a [bloomfilter](https://en.wikipedia.org/wiki/Bloom_filter) (~ 1GB).

## Is it safe?
Easypwned does not need external network access. passwords and hashes are never leaving your server.
Use the `/hash/[SHA1]` endpoint in production to avoid sending them through the network stack.

## How it works
Easypwned checks passwords based on the password list provided by [haveibeenpwned](https://haveibeenpwned.com/Passwords).
We use a bloomfilter, so it is freaking fast. The bloomfilter is generated with a chance of 1% that you get false positives.

## Endpoints
### /pw/[blank_password]
You'll get a `"secure":true` if the password is not breached.
use the /hash/ endpoint in production instead!
```
curl http://127.0.0.1:3342/pw/test
{"hash":"A94A8FE5CCB19BA61C4C0873D391E987982FBBD3","pw":"test","secure":false}
```
### /hash/[UPPERCASE(SHA1(blank_password))]
You'll get a `"secure":true` if the password is not breached.

```
curl http://127.0.0.1:3342/hash/0000000CAEF405439D57847A8657218C618160B2
{"hash":"A94A8FE5CCB19BA61C4C0873D391E987982FBBD3","pw":"test","secure":false}
```

php example
```php
(new \GuzzleHttp\Client(['base_uri' => 'localhost:3342']))->get('/hash/' . mb_strtoupper(sha1($password)));
```

## Using without docker
We build Binaries for Linux ([arm64](https://github.com/easybill/easypwned/releases/latest/download/easypwned_aarch64-unknown-linux-musl), [x86](https://github.com/easybill/easypwned/releases/latest/download/easypwned_x86_64-unknown-linux-musl
)) and OSX ([arm64](https://github.com/easybill/easypwned/releases/latest/download/easypwned_aarch64-apple-darwin), [x86](https://github.com/easybill/easypwned/releases/latest/download/easypwned_x86_64-apple-darwin)).
If you use the Binaries you need to provide the bloom filter. You could extract it from the docker container or build it on your own.


## Download the haveibeenpwned / HIBP Database (PwnedPasswordsDownloader)

We also provide a downloader for the haveibeenpwned / HIBP Database, you can build the downloader on your own or use out pre build binaries for Linux ([arm64](https://github.com/easybill/easypwned/releases/latest/download/easypwned_haveibeenpwned_downloader_aarch64-unknown-linux-musl), [x86](https://github.com/easybill/easypwned/releases/latest/download/easypwned_haveibeenpwned_downloader_x86_64-unknown-linux-musl
)) and OSX ([arm64](https://github.com/easybill/easypwned/releases/latest/download/easypwned_haveibeenpwned_downloader_aarch64-apple-darwin), [x86](https://github.com/easybill/easypwned/releases/latest/download/easypwned_haveibeenpwned_downloader_x86_64-apple-darwin))

there is also an [official downloader (PwnedPasswordsDownloader)](https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader) but it is written in c# has no pre build binaries and no support for building bloom filters on the fly.

If you download the hibp database multiple times your file would end up with different file hashes.
The order of the data will be different. the downloader needs do around a million http requests and the order of the incoming data
is directly piped to the output. You can adjust the number of the parallel requests using the argument `--parallel`. the default value is 60.

Download as Text File:
```bash
./easypwned_haveibeenpwned_downloader_aarch64-apple-darwin --sink-stdout

// you may want to pipe it to a file ...
./easypwned_haveibeenpwned_downloader_aarch64-apple-darwin --sink-stdout > hibp.txt
```

Download and Create Bloom File
```bash
./easypwned_haveibeenpwned_downloader_aarch64-apple-darwin --sink-bloom-file=easypwned.bloom
```
