<h1 align="center">awsbck</h1>

<p align="center">
  <a href="https://github.com/beeb/awsbck-rs/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/beeb/awsbck-rs/ci.yml?style=flat-square" /></a>
  <a href="https://crates.io/crates/awsbck"><img src="https://img.shields.io/crates/v/awsbck.svg?style=flat-square" /></a>
  <a href="https://github.com/beeb/awsbck-rs/blob/main/LICENSE-MIT"><img src="https://img.shields.io/crates/l/awsbck.svg?style=flat-square" /></a>
</p>

<p align="center">
  This utility lets you compress a folder and upload it to a AWS S3 bucket, once or periodically.
</p>

<hr/>

## Disclaimer

This software is in early alpha. It is not intended for production use yet. It has not been thoroughly tested yet.
The CLI options will certainly change.

## Usage

```
Usage: awsbck [OPTIONS] [FOLDER]

Arguments:
  [FOLDER]  Path to the folder to backup [env: AWSBCK_FOLDER=]

Options:
  -i, --interval <SECONDS>  Specify an interval in seconds to run the backup periodically [env: AWSBCK_INTERVAL=]
  -f, --filename <NAME>     The name of the archive that will be uploaded to S3, without extension (optional) [env: AWSBCK_FILENAME=]
  -r, --region <REGION>     The AWS S3 region [env: AWS_REGION=]
  -b, --bucket <BUCKET>     The AWS S3 bucket name [env: AWS_BUCKET=]
      --id <KEY_ID>         The AWS S3 access key ID [env: AWS_ACCESS_KEY_ID=]
  -k, --key <KEY>           The AWS S3 secret access key [env: AWS_SECRET_ACCESS_KEY=]
  -h, --help                Print help (see more with '--help')
  -V, --version             Print version
```

### Example

```shell
# The .env file in the current directory is read by awsbck
$ cat .env
AWS_REGION="eu-central-1"
AWS_ACCESS_KEY_ID="YOUR_KEY_ID"
AWS_SECRET_ACCESS_KEY="yoursecret"

$ awsbck -i 3600 -b my_bucket /my_folder
```

### Docker example

```
$ export AWS_REGION="eu-central-1"
$ export AWS_ACCESS_KEY_ID="YOUR_KEY_ID"
$ export AWS_SECRET_ACCESS_KEY="yoursecret"
$ docker run \
  --rm \
  --mount type=bind,src="$(pwd)"/target,dst=/target,readonly \
  -e AWS_REGION -e AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY \
  vbersier/awsbck:latest \
  -i 3600 -b my_bucket /target
```

## Installation

### Prebuilt binaries

Check out [the releases](https://github.com/beeb/awsbck-rs/releases) for prebuilt binaries.

### Cargo

```shell
$ cargo install awsbck
```

### Docker

This utility is available as a [docker image `vbersier/awsbck`](https://hub.docker.com/r/vbersier/awsbck).

There are two tag variants, one running as a non-root user (`latest`) and one as a root user (`root-latest`).

This image is particularly useful to backup named volumes in docker. If you encounter problems where the `awsbck` logs
report a permissions problem, then you can try to switch to the `root-latest` tag.

Below an example of using it with `docker compose`:

```yml
---
version: '3.2'

volumes:
  database:

services:
  postgresql:
    image: postgres:14
    restart: unless-stopped
    volumes:
      - type: volume
        source: database
        target: /var/lib/postgresql/data/
  awsbck:
    image: vbersier/awsbck:root-latest # postgres uses UID 999 which can't be accessed as nonroot
    restart: unless-stopped
    volumes:
      - type: volume
        source: database
        target: /database
        read_only: true
    environment:
      AWSBCK_FOLDER: /database
      AWSBCK_INTERVAL: 86400 # every 24h
      AWS_REGION: eu-central-1
      AWS_BUCKET: my_bucket
      AWS_ACCESS_KEY_ID: $AWS_ACCESS_KEY_ID
      AWS_SECRET_ACCESS_KEY: $AWS_SECRET_ACCESS_KEY
```
