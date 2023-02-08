<h1 align="center">awsbck</h1>

<p align="center">
This utility lets you compress a folder and upload it to a AWS S3 bucket, once or periodically.
</p>

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
$ cat .env
AWS_REGION="eu-central-1"
AWS_ACCESS_KEY_ID="YOUR_KEY_ID"
AWS_SECRET_ACCESS_KEY="yoursecret"

$ awsbck -i 3600 -b my_bucket /my_folder
```

## Installation

```shell
$ cargo install awsbck
```
