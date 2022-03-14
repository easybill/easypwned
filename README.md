# easypwned
Rest api to check if passwords are breached. Works offline - everything stays on your machine!

## example
The simplest way to run it is using docker:

```bash
docker run --rm --network=host timglabisch/easypwned:latest
curl http://127.0.0.1:3342/pw/[BLANK_PASSWORD]  # use /hash/SHA1 in prod apps (pw/[PW] is for testing).
curl http://127.0.0.1:3342/hash/0000001C5F765AA063E4F8470451F85F7DB4ED3A # << UPPERCASE(SHA1(PLAINTEXT))
```
The docker image comes with the bloomfilter (~ 1GB).

## is it safe?
Easypwned does not need external network access, passwords / hash's are never leaving your server.
Use the `hash/[SHA1]` endpoint in production to avoid sending through the network stack.

## how it works
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



## using without docker
You could run easypwned without docker, but than you've to build the bloomfilter by yourself.
In theorie you must rebuild the bloomfilter everytime you update easypwned because the bloomfilter might change.
Rebuilding the bloom filter is not a big deal, but takes a bit of CPU and DISK. Take a look at the Makefiletarget `make build_bloom`.
Another benefit of the docker image is, that easypwned updates stays small if the bloomfilter doesn't change.
You could also get the bloomfilter from the dockerimage, look at the projects `Dockerfile` for an example.