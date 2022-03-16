# easypwned
Rest API to check if a password is in a data breach. Works offline - everything stays on your machine!

## Example
The simplest way to run it is using docker:

```bash
docker run --rm --network=host easybill/easypwned:latest
curl http://127.0.0.1:3342/pw/[BLANK_PASSWORD]  # use /hash/SHA1 in prod apps (pw/[PW] is for testing).
curl http://127.0.0.1:3342/hash/0000001C5F765AA063E4F8470451F85F7DB4ED3A # << UPPERCASE(SHA1(PLAINTEXT))
```
The [dockerimage](https://hub.docker.com/repository/docker/easybill/easypwned) comes with a list of compromised passwords in the form of a [bloomfilter](https://en.wikipedia.org/wiki/Bloom_filter) (~ 1GB).

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
While we recommend using the Docker image, you can run Easypwned without using Docker as well. To do so, you have to build the bloomfilter yourself.
In theory you must rebuild the bloomfilter everytime you update easypwned because the bloomfilter might change.
Rebuilding the bloom filter is not a big deal, but takes a bit of CPU and DISK. Take a look at the Makefile target `build_bloom`.
Another benefit of the Docker image is, that easypwned updates stay small if the bloomfilter doesn't change.
You could also get the bloomfilter from the dockerimage. Look at the project's `Dockerfile` for an example.
