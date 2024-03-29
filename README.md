# dptran

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Version](https://img.shields.io/badge/Version-2.0.0-brightgreen.svg)

A tool to run DeepL translations on your command line.   
It's written in Rust, and uses curl to connect to the DeepL API.  
To use, you need to get the DeepL API key from [https://www.deepl.com/en/pro-api/](https://www.deepl.com/en/pro-api/).

## Binary crate
### Install

#### Use crates.io

1. Install ``rustup`` or ``cargo`` to your computer.
2. run this:
```bash
$ cargo install dptran --features=app
```

### Usage

#### Setting API key

Please be sure to get your DeepL API key (it's free!) and set it up on dptran before using the service.

```bash
$ dptran set --api-key [API key]
```

#### Translate in normal mode

```bash
$ dptran Bonjour
Hello
$ dptran -t FR Hello
Bonjour
```

It is possible to specify the source language with the ``-f`` option and the destination language with the ``-t`` option.
If you omit the destination language option, the translation will be done in English (EN-US).  

For more information about language codes, see the language list:  

```bash
$ dptran list -s    # for the list of source languages
$ dptran list -t    # for the list of target languages
```

#### Translate in interactive mode

```bash
$ dptran
> ありがとうございます。
Thank you very much.
> Ich stehe jeden Tag um 7 Uhr auf.
毎日7時に起きています。
> La reunión comienza a las 10 a.m.
The meeting begins at 10 a.m.
> 今天玩儿得真开心！
Had a great time today!
> quit
```

Multiple source texts can be translated interactively.  
Exit with ``quit``.

If you want to translate the source texts into a specific language, use the ``-t`` option. 

##### Translate multiple lines

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

#### Translate from the pipeline

You can translate the output of other commands with dptran.

e.g. Translate the content of the man page into Japanese.  

```bash
$ man ls | cat | dptran -t JA
```

#### Show help

For more information about commands, see help:  

```bash
$ dptran -h
```

#### Displays the number of characters remaining to be translated

```bash
$ dptran -u
usage: 222 / 500000 (0%)
remaining: 499778
```

You can see the number of remaining characters that can be translated by DeepL API. 
The free DeepL API plan lets you translate up to 500,000 characters per month.

### Change default target language

It is set to English (EN-US) by default.  
You can change it with ``set --target-lang``.  
For example, to change it to Japanese (JA), do the following:

```bash
$ dptran set --target-lang JA
```

### Reset settings

You can reset all settings.  
Note: The API key will be reset as well. If you wish to use dptran again, please set the API key again.  

```bash
$ dptran set --clear
```

### How to uninstall?

```bash
$ cargo uninstall dptran
```

## Library crate (v2.0.0~)
See the documentation for the library crate [here](https://docs.rs/dptran/).

### Usage
```bash
$ cargo add dptran
```
Or, add this to your Cargo.toml:
```toml
[dependencies]
dptran = "2.0.0"
```
