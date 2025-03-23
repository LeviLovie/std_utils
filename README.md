# std_utils

Some common utils I add to every project

## Usage

This crate contains traits tht add `.anyhow()` to `std::result::Result<T, String>`, `std::result::Result<T, std::error::Error>`, and `Option<T>`. Method returns `anyhow::Result<T, anyhow::Error>`.

Trait `Debugging` also implements `.log()` and `.logmsg("Msg here")` which uses tracing to log an error if it exists and return `self`, `.panic()` and `.panicmsg()` do the same but panic in case of error, `.context("Msg here")` and `.dot()` are used for backtraces, taken from [anyhow_ext](https://github.com/gensmusic/anyhow_ext/tree/main).
