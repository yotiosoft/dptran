# dptran

English | [日本語版はこちら](README_JA.md)

![Crates.io Version](https://img.shields.io/crates/v/dptran)
[![CI](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**dptran** is a command-line tool and library for using the DeepL API, written in Rust.

## Features

### Binary CLI

- Translate via CLI, interactive input, pipeline, or file/editor input
- Support for multi-line input, output to file, and removing line breaks
- DeepL API Free / Pro support
- Language code lookup and character usage tracking
- Result caching

### Library

- API client for DeepL translation
- Language code and usage queries

## Installation

### Binary crate
```bash
cargo install dptran
```

### Library crate
```bash
cargo add dptran --no-default-features
```
## Basic Usage
### Set API Key
```bash
dptran api --api-key-free [Your API key]
# or set env: export DPTRAN_DEEPL_API_KEY=[Your API key]
```

### Translate
```bash
# simple translation
dptran Hello
# translate with target language
dptran -t JA Hello
# translate with source language
dptran -f EN -t JA Hello
# translate interactively
dptran
> Hello
# translate from a file
dptran -i text.txt
# translate with a pipeline
echo "Hello" | dptran -t JA
# translate with line breaks removed
dptran -r "Hello\nWorld"
# translate from an editor (like vim, emacs. must be set in config)
dptran -e
```

### Options
- -t [LANG] Set the default target language
- -f [LANG] Set the default source language
- -o [FILE] Output to file
- -r Remove line breaks
- -u Show character usage
- list -s / -t Show available language codes

For more options and detailed usage, run:
```bash
dptran -h
```

### Configuration
Change default target language:

```bash
dptran config --target-lang JA
```
Reset all settings:

```bash
dptran config --clear-all
```

## Development & Testing
Run unit tests.
To run tests that require a real DeepL API key, set the environment variable `DPTRAN_DEEPL_API_KEY`:

```bash
export DPTRAN_DEEPL_API_KEY=[API key]
cargo test -- --test-threads=1
```

Some require dummy API server to be running.  
The dummey server will be run at `http://localhost:8000/` by default.

```bash
$ pip3 install -r requirements.txt
$ uvicorn dummy-api-server:app --reload
```

## Documentation
Crate page: https://crates.io/crates/dptran

Library docs: https://docs.rs/dptran

## License
Licensed under either of:

- MIT License
- Apache License 2.0
