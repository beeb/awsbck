# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Build System

- Bump actions/checkout from 3 to 4
- Bump robinraju/release-downloader from 1.7 to 1.8
- Bump docker/metadata-action from 4 to 5
- Bump docker/setup-buildx-action from 2 to 3
- Bump docker/setup-qemu-action from 2 to 3

### CI Workflows

- Add release-plz
- Fix missing secret

### Miscellaneous Tasks

- Update aws sdk
- Release

## [0.3.4] - 2023-08-27

### Miscellaneous Tasks

- Update flake
- Update dependencies
- Bump patch number
- Update dependencies

### Styling

- Cargo fmt

## [0.3.3] - 2023-07-13

### Build System

- Add flake for dev shell
- Add package build definition in nix file

### CI Workflows

- Change commit email
- Ignore changelog commits
- Categorize ci and build commits
- Use action to install rustfmt and clippy
- Fix format

### Documentation

- Readme

### Miscellaneous Tasks

- Update deps
- Update dependencies
- Update email
- Update deps
- Update deps
- Update dependencies
- Remove unused dependency
- Bump version number

### Refactor

- Add clippy pedantic rules and fix warns

## [0.3.2] - 2023-05-25

### Bug Fixes

- Runner

### CI Workflows

- Generate changelog automatically
- Add macos-aarch64 build target
- Revert macOS aarch64 because it doesn't work
- Try to cross-compile for apple silicon
- Freeze xcode version
- Fix release dependencies

### Miscellaneous Tasks

- Add changelog file
- Update deps
- Bump patch number

## [0.3.1] - 2023-04-12

### CI Workflows

- Disable incremental builds
- Github registry for docker
- Update docker to ghcr.io
- Invert order of images push
- Remove env vars as they are defined by the toolchain action

### Documentation

- Readme
- Readme
- Update image to ghcr.io
- Readme

### Miscellaneous Tasks

- Update env example
- Update dependencies and fix use statements
- Bump patch number

## [0.3.0] - 2023-03-11

### CI Workflows

- Directly publish release
- Disable failing task
- Test to add permissions
- Restrict permissions
- Restore cron
- Use new sparse protocol for cargo

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

## [0.2.11] - 2023-02-11

### Bug Fixes

- Sanitize default filename

### Miscellaneous Tasks

- Bump patch number

### Refactor

- Folder name sanitation

## [0.2.10] - 2023-02-11

### CI Workflows

- Make sure lockfile was updated

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

### Build System

- Simplify dockerfile

### CI Workflows

- Remove redundant matrix def

### Documentation

- Update docker-compose example
- Readme

### Miscellaneous Tasks

- Bump patch number

### Refactor

- New struct for Archive

## [0.2.7] - 2023-02-09

### Bug Fixes

- Add apostrophe as valid char

### CI Workflows

- Run tests on PR and release
- Only build if tests pass

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

## [0.2.6] - 2023-02-09

### Bug Fixes

- Image name in CI
- Better ctrl-c handling
- Better ctrl-c handling

### Build System

- Fix docker file permissions
- Fix user in dockerfile
- Add root version of docker container

### CI Workflows

- Add root docker image to release workflow

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

## [0.2.5] - 2023-02-08

### Build System

- Root in docker

### CI Workflows

- Rename workflow
- Rename workflow

### Documentation

- Readme
- Readme

### Features

- Add option to set filename of archive
- Only add file extension if it was not in arg

### Miscellaneous Tasks

- Bump version number

## [0.2.4] - 2023-02-08

### Build System

- Remove unused dependencies, enable thin LTO

### CI Workflows

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

### Documentation

- Readme

### Miscellaneous Tasks

- Bump patch number

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

### Build System

- Allow unused
- Add package metadata

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

### Wip

- Upload to dropbox
- Compressing
- Comment out unused vars
- File should probably have a fixed name so versioning works
- First prototype
- Better env handling and scheduling
- Trying to fix the ctrl-c handling

<!-- generated by git-cliff -->
