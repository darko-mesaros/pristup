# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## TODO 📋

- TODO: Use `xdg-open` to automatically open the browser (where possible)
- TODO: Generate temporary CLI credentials for sharing
- TODO: Generate temporary S3 bucket upload permissions

## [0.2.1] - 2024-04-30
### Added
- The ability to configure the timeout duration

### Changed
- Fixed some unoptimized code

## [0.2.0] - 2024-03-31
### Added
- Now installable. Will create configuration files for you in your `$HOME/.config/pristup` directory
- During the `init` process it will prompt you for the AWS Account ID and Role.

### Changed
- The way the configuration file is read. It now reads it from the predefined config directory.

## [0.1.0] - 2024-03-24
### Added
- Base functionality
- Generates an URL for temporary console access
- Handles parameters and configuration files (almost)
