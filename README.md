# dptran

English | [日本語版はこちら](README_JA.md)

![Crates.io Version](https://img.shields.io/crates/v/dptran)
[![Rust](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A tool to run DeepL translations on your command line.   
It's written in Rust, and uses curl to connect to the DeepL API.  
To use, you need to get the DeepL API key from [https://www.deepl.com/en/pro-api/](https://www.deepl.com/en/pro-api/).

# Installation
## Binary crate
``dptran`` is available on crates.io ([https://crates.io/crates/dptran](https://crates.io/crates/dptran)).

1. Install ``rustup`` or ``cargo`` to your computer.
2. run this:
```bash
$ cargo install dptran
```

## Library crate
``dptran`` includes the binary crate's dependent crates (such as ``clap``, ``serde_json`` and ``confy``) by the default features.  
To install only the library crate, please disable the default features by adding ``--no-default-features`` argument.
```bash
$ cargo add dptran --no-default-features
```
Or, add this to your Cargo.toml:
```toml
[dependencies]
dptran = { version = "2.3.0", default-features = false }
```

# Binary crate
The binary crate provides a command-line tool to translate text using the DeepL API.

## Features

- Translate text from the command line arguments.
- Translate text interactively.
- Translate multiple lines.
- Translate text from the pipeline.
- Translate text from a file. (v2.1.0-)
- Translate text from an editor. (v2.1.0-)
- Remove line breaks from the source text. (v2.1.0-)
- Output to a text file. (v2.1.0-)
- Check the number of characters remaining to be translated.
- Check valid language codes.
- Cache the translation results. (v2.1.0-)
- Support both DeepL API Free and Pro plans. (v2.3.0-)

## Usage

### Setting API key

Please be sure to get your DeepL API key and set it up on dptran before using the service.  
The API key is available for free (up to 500,000 characters per month).  
[https://www.deepl.com/en/pro-api/](https://www.deepl.com/en/pro-api/)

**For DeepL API Free plan:**
```bash
$ dptran api --api-key-free [Your API key]
```

**For DeepL API Pro plan:**
```bash
$ dptran api --api-key-pro [Your API key]
```

#### Set API Key by environment variable

Alternatively, you can set the API key in the environment variable ``DPTRAN_DEEPL_API_KEY`` (for free plan) or ``DPTRAN_DEEPL_API_KEY_PRO`` (for pro plan).  
If you set the API key in the environment variable, ``dptran`` will automatically use it.

**On Linux or macOS:**
```bash
$ export DPTRAN_DEEPL_API_KEY=[API key]
```
To set it permanently, add the above line to your ``~/.bashrc`` or ``~/.zshrc`` file.

**On Windows:**  
Set the environment variable in the system properties.

### Translate from the command line arguments

If any language is not specified, the source language is automatically detected and the destination language is set to English (EN) by default.  
It is possible to specify the source language with the ``-f`` option and the destination language with the ``-t`` option.

```bash
$ dptran Bonjour
Hello
$ dptran -t FR Hello
Bonjour
```

### Translate in interactive mode

```bash
$ dptran
> ありがとうございます。
Thank you very much.
> Ich stehe jeden Tag um 7 Uhr auf.
I get up at 7 a.m. every day.
> La reunión comienza a las 10 a.m.
The meeting begins at 10 a.m.
> 今天玩儿得真开心！
Had a great time today!
> quit
```

Multiple source texts can be translated interactively.  
Exit with ``quit``.

If you want to translate the source texts into a specific language, use the ``-t`` option. 

### Translate multiple lines

To enter multiple lines, use the -m option.  
Then enter a blank line to send the input.

```bash
$ dptran -m -t JA
> A tool to run DeepL translations on your command line.
..It's written in Rust, and uses curl to connect to the DeepL API.
..To use, you need to get the DeepL API key from https://www.deepl.com/en/pro-api/.
..
コマンドラインでDeepL翻訳を実行するためのツールです。
これはRustで書かれており、DeepL APIへの接続にはcurlを使用します。
使用するには、https://www.deepl.com/en/pro-api/ から DeepL API キーを取得する必要があります。
```

### Translate from the pipeline

You can translate the output of other commands with dptran.

e.g. Translate the content of the man page into Japanese.  

```bash
$ man ls | cat | dptran -t JA
```

### Translate from a file

You can translate the contents of a text file with dptran by using the ``-i`` option.

```bash
$ dptran -i file.txt
```

### Translate from an editor application (e.g. vi, vim, nano, emacs, etc.)

You can translate the contents from an editor with dptran by using the ``-e`` option.

#### Example: vi
```bash
$ dptran config -e vi
$ dptran -e
```

#### Example: vim
```bash
$ dptran config -e vim
$ dptran -e
```

#### Example: nano
```bash
$ dptran config -e nano
$ dptran -e
```

#### Example: emacs
```bash
$ dptran config -e "emacs -nw"
$ dptran -e
```

### Remove line breaks

You can remove line breaks from the source text with the ``-r`` option.

```bash
$ dptran -t FR -e -r
```
For example, the following input (in the editor):
```bash
Hello!
How are you?
```
will be translated as one line like this:
```bash
Bonjour, comment allez-vous ?
```

### Output to a text file

You can output the translation result to a text file with the ``-o`` option.

```bash
$ dptran -t JA Hello -o output.txt
```

### Show help

For more information about commands, see help:  

```bash
$ dptran -h
```

### Displays the number of characters remaining to be translated

```bash
$ dptran -u
usage: 222 / 500000 (0%)
remaining: 499778
```

You can see the number of remaining characters that can be translated by DeepL API in the current month.  
The free DeepL API plan lets you translate up to 500,000 characters per month.

## Language codes
If you omit the destination language option, the translation will be done in English (EN) by default.  
For more information about language codes, see the language list getting from DeepL API:  

```bash
$ dptran list -s    # for the list of source languages
 AR: Arabic     BG: Bulgarian  CS: Czech     
 DA: Danish     DE: German     EL: Greek     
 EN: English    ES: Spanish    ET: Estonian  
 FI: Finnish    FR: French     HU: Hungarian 
 ID: Indonesian IT: Italian    JA: Japanese  
 KO: Korean     LT: Lithuanian LV: Latvian   
 NB: Norwegian  NL: Dutch      PL: Polish    
 PT: Portuguese RO: Romanian   RU: Russian   
 SK: Slovak     SL: Slovenian  SV: Swedish   
 TR: Turkish    UK: Ukrainian  ZH: Chinese   
$ dptran list -t    # for the list of target languages
 AR     : Arabic                 BG     : Bulgarian             
 CS     : Czech                  DA     : Danish                
 DE     : German                 EL     : Greek                 
 EN     : English                EN-GB  : English (British)     
 EN-US  : English (American)     ES     : Spanish               
 ET     : Estonian               FI     : Finnish               
 FR     : French                 HU     : Hungarian             
 ID     : Indonesian             IT     : Italian               
 JA     : Japanese               KO     : Korean                
 LT     : Lithuanian             LV     : Latvian               
 NB     : Norwegian              NL     : Dutch                 
 PL     : Polish                 PT     : Portuguese            
 PT-BR  : Portuguese (Brazilian) PT-PT  : Portuguese (European) 
 RO     : Romanian               RU     : Russian               
 SK     : Slovak                 SL     : Slovenian             
 SV     : Swedish                TR     : Turkish               
 UK     : Ukrainian              ZH     : Chinese (simplified)  
 ZH-HANS: Chinese (simplified)   ZH-HANT: Chinese (traditional)
```

## Change default target language

It is set to English (EN) by default.  
You can change it with ``set --target-lang``.  
For example, to change it to Japanese (JA), do the following:

```bash
$ dptran config --target-lang JA
```

## Reset settings

You can reset all settings.  
Note: The API key will be reset as well. If you wish to use dptran again, please set the API key again.  

```bash
$ dptran config --clear-all
```

## Uninstall

```bash
$ cargo uninstall dptran
```

# Library crate (v2.0.0-)
See the documentation for the library crate [here](https://docs.rs/dptran/).

## Features

- Translate text.
- Check the number of characters remaining to be translated.
- Check valid language codes.
- Support both DeepL API Free and Pro plans. (v2.3.0-)

# Running tests

Some tests use the dummy API server.
To run the tests, you need to start the dummy API server implemented in ``dummy-api-server.py``.

```bash
$ pip3 install -r requirements.txt
$ uvicorn dummy-api-server:app --reload
```

The dummy API server will run on ``http://localhost:8000/`` by default.

Also, some unittests require a real DeepL API key.
To run the tests, set the environment variable ``DPTRAN_DEEPL_API_KEY`` (for free plan).

```bash
$ export DPTRAN_DEEPL_API_KEY=[API key]
```

Then, run the tests.  
You should run the tests with ``--test-threads=1`` because the DeepL API has a limit on the number of requests per second.

```bash
$ cargo test -- --test-threads=1
```

# License
This project is licensed under the MIT License and Apache License 2.0.
