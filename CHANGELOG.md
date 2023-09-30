# Changelog

All notable changes to this project will be documented in this file.


## [Unreleased]

### Added

- Add release-plz

### Changed

- Update aws sdk
- Bump actions/checkout from 3 to 4
- Bump robinraju/release-downloader from 1.7 to 1.8
- Bump docker/metadata-action from 4 to 5
- Bump docker/setup-buildx-action from 2 to 3
- Bump docker/setup-qemu-action from 2 to 3
- Update config
- Adjust git-cliff config for release-plz compatibility
- Adjust changelog format

### Fixed

- Fix missing secret
- Fix changelog format

## [0.3.4] - 2023-08-27

### Changed

- Update flake
- Update dependencies
- Cargo fmt
- Bump patch number
- Update dependencies

## [0.3.3] - 2023-07-13

### Added

- Add clippy pedantic rules and fix warns
- Add flake for dev shell
- Add package build definition in nix file

### Changed

- Update deps
- Update dependencies
- Update email
- Update deps
- Update deps
- Change commit email
- Ignore changelog commits
- Categorize ci and build commits
- Update dependencies
- Use action to install rustfmt and clippy
- Bump version number

### Documentation

- Readme

### Fixed

- Fix format

### Removed

- Remove unused dependency

## [0.3.2] - 2023-05-25

### Added

- Add changelog file
- Add macos-aarch64 build target

### Changed

- Generate changelog automatically
- Update deps
- Revert macOS aarch64 because it doesn't work
- Try to cross-compile for apple silicon
- Freeze xcode version
- Bump patch number

### Fixed

- Runner
- Fix release dependencies

## [0.3.1] - 2023-04-12

### Changed

- Disable incremental builds
- Update env example
- Github registry for docker
- Update docker to ghcr.io
- Invert order of images push
- Update dependencies and fix use statements
- Bump patch number

### Documentation

- Readme
- Readme
- Update image to ghcr.io
- Readme

### Removed

- Remove env vars as they are defined by the toolchain action

## [0.3.0] - 2023-03-11

### Added

- Add logging

### Changed

- Directly publish release
- Use AsRef<Path> where possible
- Update deps
- Cargo update
- Disable failing task
- Test to add permissions
- Restrict permissions
- Restore cron
- Use new sparse protocol for cargo
- Use cron expression instead of interval
- Bump minor version number (breaking)

### Documentation

- Comments
- Comment
- Readme

### Fixed

- Check cron backup

### Removed

- Remove unused commented code

## [0.2.11] - 2023-02-11

### Changed

- Folder name sanitation
- Bump patch number

### Fixed

- Sanitize default filename

## [0.2.10] - 2023-02-11

### Added

- Added end-to-end test using S3Mock in a container
- Add filename arg test
- Add docker badge

### Changed

- Comment indentation
- E2e test
- Make sure lockfile was updated
- Update deps
- Use clap for args validation (required / default value)
- Bump patch number

### Documentation

- Readme
- Root not needed with new container setup

### Fixed

- Safer value for filesize
- Refactor size check

## [0.2.9] - 2023-02-09

### Added

- Add special char test case

### Changed

- Bump patch number

### Fixed

- Validate filename length

## [0.2.8] - 2023-02-09

### Changed

- Simplify dockerfile
- New struct for Archive
- Bump patch number

### Documentation

- Update docker-compose example
- Readme

### Removed

- Remove redundant matrix def

## [0.2.7] - 2023-02-09

### Added

- Add apostrophe as valid char

### Changed

- Sanitize the desired filename
- Log filename after upload
- Allow forward slash to enable sub-folders
- Run tests on PR and release
- Only build if tests pass
- Bump patch number

### Documentation

- Readme
- Readme

### Fixed

- Filename sanitize

## [0.2.6] - 2023-02-09

### Added

- Add root version of docker container
- Add root docker image to release workflow
- Add docker example
- Add badges
- Add comments in code

### Changed

- Docker workflow
- Update repo url
- Bump patch number

### Documentation

- Readme
- Readme
- Readme

### Fixed

- Fix docker file permissions
- Image name in CI
- Fix user in dockerfile
- Better ctrl-c handling
- Better ctrl-c handling

## [0.2.5] - 2023-02-08

### Added

- Add option to set filename of archive

### Changed

- Rename workflow
- Rename workflow
- Root in docker
- Only add file extension if it was not in arg
- Bump version number

### Documentation

- Readme
- Readme

## [0.2.4] - 2023-02-08

### Added

- Add linting
- Add builds and artifact cleanup
- Add publish to crates.io

### Changed

- Combine all checks into 1 job
- Test run
- Conditional build
- Format
- Full release flow
- Bump patch number

### Documentation

- Readme

### Fixed

- Fix format
- Fix yaml
- Fix path to armv7 binary
- Fix crates.io publish

### Removed

- Remove unused dependencies, enable thin LTO

## [0.2.3] - 2023-02-08

### Changed

- Code cleanup and separation
- Bump patch number

## [0.2.2] - 2023-02-08

### Added

- Add logging

### Changed

- Use temp dir for temp file
- More detailled logging
- Bump patch number

## [0.2.1] - 2023-02-08

### Added

- Add dual license

### Changed

- Update lockfile
- Bump patch number and update metadata

### Documentation

- Readme
- Readme
- Readme
- Readme

### Fixed

- License metadata

## [0.2.0] - 2023-02-08

### Added

- Add dropbox token arg
- Add test to list directory
- Add readme
- Add package metadata
- Add example env file

### Changed

- Parse arguments and setup scheduler
- Better error handling when path is wrong
- Better error handling and tidier structure
- Renamed config module
- Upload to dropbox
- Format
- Compressing
- Use metadata
- Allow unused
- Move compression login into fn
- Error context
- Comment out unused vars
- Get mtime after compression
- File should probably have a fixed name so versioning works
- First prototype
- Multipart upload
- Better env handling and scheduling
- Rename project
- Trying to fix the ctrl-c handling
- Drop external dependencies for interval
- Bump minor version number

### Removed

- Remove archive after uplaod

