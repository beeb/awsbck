<h1 align="center">awsbck</h1>

<p align="center">
  <a href="https://github.com/beeb/awsbck/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/beeb/awsbck/ci.yml?style=flat-square" /></a>
  <a href="https://crates.io/crates/awsbck"><img src="https://img.shields.io/crates/v/awsbck.svg?style=flat-square" /></a>
  <a href="https://github.com/beeb/awsbck/blob/main/LICENSE-MIT"><img src="https://img.shields.io/crates/l/awsbck.svg?style=flat-square" /></a>
</p>

<p align="center">
  This utility lets you compress a folder and upload it to a AWS S3 bucket, once or periodically.
</p>

<hr/>

## Disclaimer

This software is in a beta stage and, although it has not caused any problems in testing, I wouldn't recommend it for
production use.

Use at your own risks!

The CLI will certainly change, but any breaking change should mean an increase in the minor version number as per semver
, until it reaches `1.0.0`. New features that are backwards-compatible and bug fixes will lead to patch number bumps
until then.

## Usage

```
Usage: awsbck [OPTIONS] --bucket <BUCKET> --id <KEY_ID> --key <KEY> <FOLDER>

Arguments:
  <FOLDER>  Path to the folder to backup [env: AWSBCK_FOLDER=]

Options:
  -c, --cron <EXPR>      Specify a cron espression to run the backup on a schedule [env: AWSBCK_CRON=]
  -f, --filename <NAME>  The name of the archive that will be uploaded to S3, without extension (optional) [env: AWSBCK_FILENAME=]
  -r, --region <REGION>  The AWS S3 region [env: AWS_REGION=] [default: us-east-1]
  -b, --bucket <BUCKET>  The AWS S3 bucket name [env: AWS_BUCKET=]
      --id <KEY_ID>      The AWS S3 access key ID [env: AWS_ACCESS_KEY_ID=]
  -k, --key <KEY>        The AWS S3 secret access key [env: AWS_SECRET_ACCESS_KEY=]
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

CLI arguments take precedence over environment variables.

The cron expression is parsed by the [`cron`](https://github.com/zslayton/cron) crate, with the following format (year
is optional):

```rust
//                sec, min, hour,   day of month, month,  day of week, year
let expression = "0    30   9,12,15 1,15          May-Aug Mon,Wed,Fri  2018/2";
```

The `--filename` option accepts ASCII alphanumeric characters and `!-_.*'()/`. Other characters will be discarded.

### Example

```shell
# The .env file in the current directory is read by awsbck
$ cat .env
AWS_REGION="eu-central-1"
AWS_ACCESS_KEY_ID="YOUR_KEY_ID"
AWS_SECRET_ACCESS_KEY="yoursecret"

$ awsbck -c "@hourly" -b my_bucket /my_folder
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
  ghcr.io/beeb/awsbck:latest \
  -c "15 */10 * * * *" -b my_bucket /target
```

## Installation

### Prebuilt binaries

Check out [the releases](https://github.com/beeb/awsbck/releases) for prebuilt binaries.

### Cargo

```shell
$ cargo install awsbck
```

### Nix

Available through nixpkgs on the unstable channel.

```shell
$ nix-env -iA nixpkgs.awsbck
```

### Docker

This utility is available as a
[docker image `ghcr.io/beeb/awsbck`](https://github.com/beeb/awsbck/pkgs/container/awsbck).

There are two tag variants, one running as a non-root user (`latest`) and one as a root user (`root-latest`).

This image is particularly useful to backup named volumes in docker. If you encounter problems where the `awsbck` logs
report a permissions problem, then you can try to switch to the `root-latest` tag.

Below an example of using it with `docker compose`. In order to make sure the backup happens properly, we can't just
copy the db data, as it might be in the middle of a write or other operation. Thus we send the `pg_dumpall` command
and store the resulting dump to a separate volume that we can backup to S3.

```yml
---
version: '3.2'

volumes:
  # the first volume is to persist the database raw data
  database:
  # this volume will be used to share the dump file with awsbck
  database-backup:

services:
  postgresql:
    image: postgres:14
    restart: unless-stopped
    volumes:
      - type: volume
        source: database
        target: /var/lib/postgresql/data/
      - type: volume
        source: database-backup
        target: /backup
  # this service will send a dump command to the postgres container periodically (here 6h)
  # and store the resulting file in the `database-backup` volume mounted at `/backup`
  postgres-backup:
    image: docker:cli
    container_name: postgres_backup
    volumes:
      - type: bind
        source: /var/run/docker.sock
        target: /var/run/docker.sock
    command:
      [
        '/bin/sh',
        '-c',
        'while true; do sleep 21600; docker exec -t postgres pg_dumpall -c -U postgres > /backup/dump_database.sql; done'
      ]
  # we mount the backup volume as read-only and back up the SQL dump daily at 3.12am
  awsbck:
    image: ghcr.io/beeb/awsbck:latest
    restart: unless-stopped
    volumes:
      - type: volume
        source: database-backup
        target: /database
        read_only: true
    environment:
      AWSBCK_FOLDER: /database
      AWSBCK_CRON: '0 12 3 * * * *'
      AWS_REGION: eu-central-1
      AWS_BUCKET: my_bucket
      AWS_ACCESS_KEY_ID: $AWS_ACCESS_KEY_ID
      AWS_SECRET_ACCESS_KEY: $AWS_SECRET_ACCESS_KEY
```
