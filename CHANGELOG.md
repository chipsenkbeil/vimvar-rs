# Changelog

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.3.0] - 2022-04-17

### Added

- Support for `init.lua` files for neovim
- Add explicit testing of `search::find_vimrc()`

### Changed

- Refactor `utils::find_vimrc()` to `search::find_vimrc()`

## [0.2.0] - 2021-05-31

### Changed

- Refactor helper methods to be separate from `VimVar` struct (breaking)

## [0.1.1] - 2021-05-30

### Fixed

- Using `+set nonumber` to command to avoid capturing lines in print
- Handle exit code of 1 from neovim

## [0.1.0] - 2021-05-29

### Added

- Initial release that supports extracting variables from vim and neovim

