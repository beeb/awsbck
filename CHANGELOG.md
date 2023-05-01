# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Bug Fixes

- Runner

### Miscellaneous Tasks

- Add changelog file

### Ci

- Generate changelog automatically

## [0.3.1] - 2023-04-12

### Documentation

- Readme
- Readme
- Update image to ghcr.io
- Readme

### Miscellaneous Tasks

- Update env example
- Update dependencies and fix use statements
- Bump patch number

### Ci

- Disable incremental builds
- Github registry for docker
- Update docker to ghcr.io
- Invert order of images push
- Remove env vars as they are defined by the toolchain action

## [0.3.0] - 2023-03-11

### Documentation

- Comments
- Comment
- Readme

### Features

- Use cron expression instead of interval
- Add logging

### Miscellaneous Tasks

- Update deps
- Cargo update
- Bump minor version number (breaking)

### Refactor

- Use AsRef<Path> where possible

### Styling

- Remove unused commented code

### Testing

- Check cron backup

### Ci

- Directly publish release
- Disable failing task
- Test to add permissions
- Restrict permissions
- Restore cron
- Use new sparse protocol for cargo

## [0.2.11] - 2023-02-11

### Bug Fixes

- Sanitize default filename

### Miscellaneous Tasks

- Bump patch number

### Refactor

- Folder name sanitation

## [0.2.10] - 2023-02-11

### Documentation

- Readme
- Add docker badge
- Root not needed with new container setup

### Miscellaneous Tasks

- Update deps
- Bump patch number

### Refactor

- Use clap for args validation (required / default value)

### Styling

- Comment indentation

### Testing

- Added end-to-end test using S3Mock in a container
- Add filename arg test
- Safer value for filesize
- Refactor size check

### Ci

- Make sure lockfile was updated

### Wip

- E2e test

## [0.2.9] - 2023-02-09

### Bug Fixes

- Validate filename length

### Miscellaneous Tasks

- Bump patch number

### Testing

- Add special char test case

## [0.2.8] - 2023-02-09

### Documentation

- Update docker-compose example
- Readme

### Miscellaneous Tasks

- Bump patch number

### Refactor

- New struct for Archive

### Build

- Simplify dockerfile

### Ci

- Remove redundant matrix def

## [0.2.7] - 2023-02-09

### Bug Fixes

- Add apostrophe as valid char

### Documentation

- Readme
- Readme

### Features

- Sanitize the desired filename
- Log filename after upload
- Allow forward slash to enable sub-folders

### Miscellaneous Tasks

- Bump patch number

### Testing

- Filename sanitize

### Ci

- Run tests on PR and release
- Only build if tests pass

## [0.2.6] - 2023-02-09

### Bug Fixes

- Image name in CI
- Better ctrl-c handling
- Better ctrl-c handling

### Documentation

- Add docker example
- Readme
- Readme
- Readme
- Add badges
- Add comments in code

### Miscellaneous Tasks

- Update repo url
- Bump patch number

### Styling

- Docker workflow

### Build

- Fix docker file permissions
- Fix user in dockerfile
- Add root version of docker container

### Ci

- Add root docker image to release workflow

## [0.2.5] - 2023-02-08

### Documentation

- Readme
- Readme

### Features

- Add option to set filename of archive
- Only add file extension if it was not in arg

### Miscellaneous Tasks

- Bump version number

### Build

- Root in docker

### Ci

- Rename workflow
- Rename workflow

## [0.2.4] - 2023-02-08

### Documentation

- Readme

### Miscellaneous Tasks

- Bump patch number

### Build

- Remove unused dependencies, enable thin LTO

### Ci

- Add linting
- Combine all checks into 1 job
- Fix format
- Add builds and artifact cleanup
- Test run
- Conditional build
- Format
- Full release flow
- Fix yaml
- Fix path to armv7 binary
- Add publish to crates.io
- Fix crates.io publish

## [0.2.3] - 2023-02-08

### Miscellaneous Tasks

- Bump patch number

### Refactor

- Code cleanup and separation

## [0.2.2] - 2023-02-08

### Features

- Add logging
- Use temp dir for temp file
- More detailled logging

### Miscellaneous Tasks

- Bump patch number

## [0.2.1] - 2023-02-08

### Bug Fixes

- License metadata

### Documentation

- Readme
- Readme
- Readme
- Readme

### Miscellaneous Tasks

- Add dual license
- Update lockfile
- Bump patch number and update metadata

## [0.2.0] - 2023-02-08

### Documentation

- Add readme
- Add example env file

### Features

- Parse arguments and setup scheduler
- Better error handling when path is wrong
- Better error handling and tidier structure
- Add dropbox token arg
- Get mtime after compression
- Multipart upload
- Remove archive after uplaod
- Drop external dependencies for interval

### Miscellaneous Tasks

- Bump minor version number

### Refactor

- Renamed config module
- Use metadata
- Move compression login into fn
- Error context
- Rename project

### Styling

- Format

### Testing

- Add test to list directory

### Build

- Allow unused
- Add package metadata

### Wip

- Upload to dropbox
- Compressing
- Comment out unused vars
- File should probably have a fixed name so versioning works
- First prototype
- Better env handling and scheduling
- Trying to fix the ctrl-c handling

<!-- generated by git-cliff -->
