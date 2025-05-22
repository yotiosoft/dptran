# dptran

[English version](README.md) | 日本語

![Crates.io Version](https://img.shields.io/crates/v/dptran)
[![Rust](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

コマンドラインでDeepL翻訳を実行するツールです。  
Rustで記述されており、DeepL APIへの接続にはcurlを使用しています。  
ご利用の前に、[https://www.deepl.com/en/pro-api/](https://www.deepl.com/en/pro-api/) からのDeepL APIキーの取得が必要です。

# インストール
## Binary crate
``dptran``はcrates.ioからインストール可能です ([https://crates.io/crates/dptran](https://crates.io/crates/dptran))。

1. ``rustup`` または ``cargo`` をご使用の環境にインストールします。
2. cargo で dptran をインストールします。
```bash
$ cargo install dptran
```

## Library crate
``dptran`` は、既定でバイナリクレート用の依存クレート（``clap``、``serde_json``、``confy``など）を含みます。  
ライブラリクレートのみをインストールする場合は、引数``--no-default-features``で default feature を無効にしてください。
```bash
$ cargo add dptran --no-default-features
```
または、Cargo.toml に下記を追加してください。
```toml
[dependencies]
dptran = { version = "2.3.0", default-features = false }
```

# Binary crate
バイナリクレートは DeepL API を使用してテキストを翻訳できるコマンドラインツールを提供します。

## 機能

- コマンドライン引数からのテキスト翻訳
- テキストを対話形式で翻訳（intractive mode）
- 複数行を翻訳
- パイプラインからテキスト翻訳
- ファイルからテキスト翻訳 (v2.1.0-)
- エディタからテキスト翻訳 (v2.1.0-)
- 入力原文から改行を除去 (v2.1.0-)
- テキストファイルに翻訳結果を出力 (v2.1.0-)
- 残りの DeepL API の翻訳可能文字数を確認
- DeepL API の言語コード一覧の取得
- 翻訳結果のキャッシュ (v2.1.0-)

## 利用方法

### APIキーの設定

ご利用の前に、必ずDeepL APIキーを取得し、dptranに設定してください。  
APIキーは無料で取得可能です（月間50万文字まで）  
[https://www.deepl.com/en/pro-api/](https://www.deepl.com/en/pro-api/)

**DeepL API Free プランの場合:**
```bash
$ dptran set --api-key-free [Your API key]
```

**DeepL API Pro プランの場合:**
```bash
$ dptran set --api-key-pro [Your API key]
```

#### 環境変数でAPIキーを設定

環境変数 ``DPTRAN_DEEPL_API_KEY`` (freeプラン用) または ``DPTRAN_DEEPL_API_KEY_PRO`` (proプラン用) にAPIキーを設定することもできます。

**Linux / macOSの場合:**
```bash
$ export DPTRAN_DEEPL_API_KEY=[API key]
```
永続的に設定するには、上記の行を ``~/.bashrc`` または ``~/.zshrc`` ファイルに追加してください。

**Windowsの場合:**
システムプロパティで環境変数を設定してください。

### コマンドライン引数から翻訳

言語が指定されていない場合、翻訳元言語は自動的に検出され、翻訳先言語はデフォルトで英語（EN）に設定されます。  
``-f``オプションで入力原文の言語を、``-t``オプションで翻訳先の言語を指定することができます。

```bash
$ dptran Bonjour
Hello
$ dptran -t FR Hello
Bonjour
```

### 対話形式で翻訳

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

複数の原文をインタラクティブに翻訳できます。  
``quit`` と入力すると終了します。

翻訳先の言語を指定する場合は、``-t`` オプションで指定可能です。

### 複数行を一度に翻訳

複数行を入力するには、``-m`` オプションをご利用ください。  
入力が完了したら、空行のまま Enter キーを押してください。

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

### パイプラインから翻訳

他のコマンドの出力をパイプラインから翻訳できます。

例：manページの内容を日本語に翻訳する  

```bash
$ man ls | cat | dptran -t JA
```

### ファイルから翻訳

テキストファイルの内容を ``-i`` オプションで翻訳できます。

```bash
$ dptran -i file.txt
```

### エディタアプリから翻訳 (vi, vim, nano, emacs など)

エディタからの入力を ``-e`` オプションで翻訳できます。

#### 例: vi
```bash
$ dptran set -e vi
$ dptran -e
```

#### 例: vim
```bash
$ dptran set -e vim
$ dptran -e
```

#### 例: nano
```bash
$ dptran set -e nano
$ dptran -e
```

#### 例: emacs
```bash
$ dptran set -e "emacs -nw"
$ dptran -e
```

### 入力文から改行を削除

``-r``オプションでソーステキストから改行を取り除きます。

```bash
$ dptran -t FR -e -r
```
例えば、エディタからの入力文が以下のような場合：
```bash
Hello!
How are you?
```
これは以下のように1行にまとめて翻訳されます：
```bash
Bonjour, comment allez-vous ?
```

### 翻訳結果のファイル出力

``-o`` オプションで翻訳結果をテキストファイルに出力できます。

```bash
$ dptran -t JA Hello -o output.txt
```

### ヘルプの表示

コマンドの詳細については、ヘルプをご覧ください。 

```bash
$ dptran -h
```

### 残りの翻訳可能文字数の表示

```bash
$ dptran -u
usage: 222 / 500000 (0%)
remaining: 499778
```

現在の月で DeepL API で翻訳可能な残りの文字数を確認できます。  
無料のDeepL APIプランでは、月間50万文字まで翻訳できます。

## Language codes
翻訳先の言語オプションを省略すると、デフォルトでは英語（EN）で翻訳されます。  
言語コードの一覧を取得するには、以下のコマンドをご利用ください。

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

## デフォルトの翻訳先言語を変更する

デフォルトでは英語 (EN) に設定されています。  
これは ``set --target-lang`` で変更できます。  
例えば、日本語 (JA) に変更するには以下のようにします：

```bash
$ dptran set --target-lang JA
```

## 設定のリセット

すべての設定をリセットできます。  
注意：APIキーもリセットされます。再度dptranを使用する場合は、APIキーを再設定してください。

```bash
$ dptran set --clear
```

## アンインストール

```bash
$ cargo uninstall dptran
```

# Library crate (v2.0.0-)
library crate に関するドキュメントは[こちら](https://docs.rs/dptran/)をご参照ください。
