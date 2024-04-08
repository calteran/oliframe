# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/calteran/oliframe/compare/v0.1.0...HEAD)

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
