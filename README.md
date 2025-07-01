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

If you want to use DeepL Pro API key, set it like this:
```bash
dptran api --api-key-pro [Your API key]
# or set env: export DPTRAN_DEEPL_API_KEY_PRO=[Your API key]
```

### Translate
```bash
# simple translation (translate to default target language)
dptran Hello
こんにちは

# translate with target language
dptran -t FR Hello
Bonjour

# translate with source language
dptran -f EN -t JA Hello
こんにちは

# translate interactively
dptran -t JA
> Hello
こんにちは

# translate from a file and output to another file
dptran -i text.txt -o translated.txt
# The file `translated.txt` will contain the translated text.

# translate with a pipeline
echo "Hello" | dptran -t ZH
您好

# translate with line breaks removed
dptran -r "Hello
everyone!"
皆さん、こんにちは！

# translate from an editor (like vim, emacs. must be set in config)
dptran -e
# Editor will open
```

### Options
- -t [LANG] Set the default target language
- -f [LANG] Set the default source language
- -i [FILE] Input from file
- -o [FILE] Output to file
- -r Remove line breaks
- -u Show character usage
- list -s / -t Show available language codes

For more options and detailed usage, run:
```bash
dptran -h
```

### Subcommands

- `list`   : Show list of supported languages
- `config` : General settings such as default target language and editor command
- `api`    : API settings such as API keys and endpoint URLs
- `cache`  : Cache settings such as enabling/disabling cache, setting max entries, and clearing cache
- `help`   : Print this message or the help of the given subcommand(s)

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

## Release Notes

- v.2.3.1 (2025-07-01)
  - Binary CLI & Library: Any api endpoint can be used
  - Binary CLI: Split `set` subcommand into `config`, `api`, and `cache` subcommands
  - Test: Use the python dummy API server for some tests
