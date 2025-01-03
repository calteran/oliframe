# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/calteran/oliframe/compare/v0.1.0...HEAD)

## [0.2.4](https://github.com/calteran/oliframe/compare/v0.2.3...v0.2.4) - 2025-01-03

### Other

- *(deps)* bump clap from 4.5.21 to 4.5.23
- *(deps)* bump env_logger from 0.11.5 to 0.11.6
- *(deps)* bump thiserror from 2.0.3 to 2.0.9
- *(deps)* bump xxhash-rust from 0.8.12 to 0.8.15

## [0.2.3](https://github.com/calteran/oliframe/compare/v0.2.2...v0.2.3) - 2024-12-02

### Other

- *(deps)* bump clap from 4.5.20 to 4.5.21
- *(deps)* bump image from 0.25.4 to 0.25.5
- *(deps)* bump thiserror from 1.0.66 to 2.0.3
- *(deps)* bump tempfile from 3.13.0 to 3.14.0
- adjust `deny.toml` for updated licenses & version dups

## [0.2.2](https://github.com/calteran/oliframe/compare/v0.2.1...v0.2.2) - 2024-11-05

### Other

- *(deps)* bump clap from 4.5.19 to 4.5.20
- *(deps)* bump image from 0.25.2 to 0.25.4
- *(deps)* bump regex from 1.11.0 to 1.11.1
- *(deps)* bump thiserror from 1.0.64 to 1.0.66

## [0.2.1](https://github.com/calteran/oliframe/compare/v0.2.0...v0.2.1) - 2024-10-02

### Other

- *(deps)* bump the crate-deps group with 4 updates

## [0.2.0](https://github.com/calteran/oliframe/compare/v0.1.7...v0.2.0) - 2024-09-06

### Added

- [**breaking**] complete rewrite of the project, including improved error handling, better CLI, and more frame
  configuration options
- Full test coverage

## [0.1.7](https://github.com/calteran/oliframe/compare/v0.1.6...v0.1.7) - 2024-08-19

### Other

- change to monthly, grouped dependabot updates
- Simplify deny.toml and update allowed licenses
- *(deps)* bump csscolorparser from 0.6.2 to 0.7.0
- *(deps)* bump clap from 4.5.8 to 4.5.16
- *(deps)* bump EmbarkStudios/cargo-deny-action from 1 to 2
- *(deps)* bump image from 0.25.1 to 0.25.2

## [0.1.6](https://github.com/calteran/oliframe/compare/v0.1.5...v0.1.6) - 2024-07-01

### Other

- *(deps)* bump clap from 4.5.4 to 4.5.8

## [0.1.5](https://github.com/calteran/oliframe/compare/v0.1.4...v0.1.5) - 2024-04-29

### Fixed

- skip alpha processing on JPEG outputs

## [0.1.4](https://github.com/calteran/oliframe/compare/v0.1.3...v0.1.4) - 2024-04-14

### Fixed

- downgrade `image` due to upline change to `zune-jpeg`

## [0.1.3](https://github.com/calteran/oliframe/compare/v0.1.2...v0.1.3) - 2024-04-08

### Other

- *(deps)* bump clap from 4.5.1 to 4.5.4
- *(deps)* bump image from 0.24.9 to 0.25.1
- *(deps)* bump rayon from 1.8.1 to 1.10.0

## [0.1.2](https://github.com/calteran/oliframe/compare/v0.1.1...v0.1.2) - 2024-02-27

### Other

- *(deps)* bump image from 0.24.8 to 0.24.9
- *(deps)* bump clap from 4.4.18 to 4.5.1

## [0.1.1](https://github.com/calteran/oliframe/compare/v0.1.0...v0.1.1) - 2024-01-25

### Fixed

- dependabot.yml syntax

### Other

- *(deps)* bump rayon from 1.8.0 to 1.8.1
- *(deps)* bump clap from 4.4.12 to 4.4.18
- *(deps)* bump image from 0.24.7 to 0.24.8
- add roadmap section to README.md; mention future --watch option

## [0.1.0](https://github.com/calteran/oliframe/releases/tag/v0.1.0) - 2024-01-16

### Added

- reexport `image` and `csscolorparser::Color`
- Add support for border color in add_border function
- Rename project to "oliframe" and update references
- Add crossterm dependency and implement overwrite validation
- save images after border is applied
- Add support for accepting multiple file formats
- Add clap dependency and parser module

### Other

- split workflow into 2 jobs; add cargo deny
- *(deps)* remove indicatif & atty from dependencies
- setup github actions & dependabot
- documentation cleanup
- enhanced documentation
- significant rework of project structure and organization
- add sample files for testing
- update .gitignore to include sample photos
- update readme
- Initial commit
