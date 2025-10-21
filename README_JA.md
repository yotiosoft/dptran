# dptran

[English](README.md) | 日本語版

![Crates.io Version](https://img.shields.io/crates/v/dptran)
[![CI](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**dptran** は、Rust で書かれた DeepL API を使用するためのコマンドラインツールおよびライブラリです。

## 機能

### バイナリCLI

- CLI、対話型入力、パイプライン、またはファイル/エディタ入力による翻訳
- 複数行入力、ファイルへの出力、改行の削除をサポート
- DeepL API Free / Pro をサポート
- 言語コードの検索と文字使用状況の追跡
- 結果のキャッシュ

### ライブラリ

- DeepL 翻訳用の API クライアント
- 言語コードと使用状況のクエリ

## インストール

### バイナリクレート

```bash
cargo install dptran
```

### ライブラリクレート

``dptran`` にはデフォルトでバイナリCLIの依存関係が含まれています。
ライブラリのみを使用する場合は、デフォルトの機能を無効にしてください。

```bash
cargo add dptran --no-default-features
```
## 基本的な使い方
### APIキーの設定
```bash
dptran api --api-key-free [APIキー]
# または環境変数を設定: export DPTRAN_DEEPL_API_KEY=[APIキー]
```

DeepL Pro APIキーを使用する場合は、以下のように設定します。

```bash
dptran api --api-key-pro [APIキー]
# または環境変数を設定: export DPTRAN_DEEPL_API_KEY_PRO=[APIキー]
```

### 翻訳
```bash
# 通常の翻訳 (デフォルトのターゲット言語に翻訳)
dptran Hello
こんにちは

# ターゲット言語を指定して翻訳
dptran -t JA Hello
Bonjour

# ソース言語を指定して翻訳
dptran -f EN -t JA Hello
こんにちは

# 対話的に翻訳 (原文は入力せずに起動)
dptran -t JA
> Hello
こんにちは
> /to FR   # ターゲット言語をフランス語に変更
> Hello
Bonjour
> /quit    # 対話モードを終了

# ファイルから翻訳し別のファイルに出力
dptran -i text.txt -o translated.txt
# translated.txt ファイルに翻訳結果が保存される

# パイプラインで翻訳
echo "Hello" | dptran -t JA
您好

# 改行を削除して翻訳
dptran -r "Hello
everyone!"
皆さん、こんにちは！

# エディタから翻訳 (vim, emacs など 設定で指定する必要あり)
dptran -e
# エディタが開かれる
```

### オプション
- -t [LANG] デフォルトのターゲット言語を設定
- -f [LANG] デフォルトのソース言語を設定
- -i [FILE] ファイルから入力
- -o [FILE] ファイルに出力
- -r 改行を削除
- -u 文字使用状況を表示

その他のオプションと詳細な使用法については、以下を実行してください。
```bash
dptran -h
```

### サブコマンド

- `list`   : サポートされている言語のリストを表示 (-s はソース言語、-t はターゲット言語)
- `config` : デフォルトのターゲット言語やエディタコマンドなどの一般設定
- `api`    : APIキーやエンドポイントURLなどのAPI設定
- `cache`  : キャッシュの有効/無効、最大エントリ数の設定、キャッシュのクリアなどのキャッシュ設定
- `help`   : このメッセージまたは指定されたサブコマンドのヘルプを表示

### 設定
デフォルトのターゲット言語を変更:

```bash
dptran config --target-lang JA
```
すべての設定をリセット:

```bash
dptran config --clear-all
```

### API エンドポイントの設定

API エンドポイント URL を設定するには、`api` サブコマンドを使用します。

```bash
dptran api --endpoint-of-translation <ENDPOINT_OF_TRANSLATION>
dptran api --endpoint-of-usage <ENDPOINT_OF_USAGE>
dptran api --endpoint-of-langs <ENDPOINT_OF_LANGUAGES>
```

以降、お好みの API エンドポイントを使用できます。（例：ローカル LLM サーバ）  
API エンドポイントは DeepL API の仕様との互換性が必要です。

## 開発とテスト
単体テストを実行します。
実際の DeepL API キーを必要とするテストを実行するには、環境変数 `DPTRAN_DEEPL_API_KEY` を設定します。

```bash
export DPTRAN_DEEPL_API_KEY=[APIキー]
cargo test -- --test-threads=1
```

一部のテストでは、ダミー API サーバが実行されている必要があります。
ダミーサーバは既定で `http://localhost:8000/` で実行されます。

```bash
pip3 install -r requirements.txt
uvicorn dummy_api_server.main:app --reload
```

## ドキュメント
crate page : https://crates.io/crates/dptran

ライブラリドキュメント: https://docs.rs/dptran

## ライセンス
以下のいずれかのライセンスで提供されます。

- MIT License
- Apache License 2.0

## リリースノート

- v2.3.4 (2025-10-04)
  - バイナリ CLI: interactive モードでのコマンド入力をサポート（`/quit`, `/help`, `/from`, `/to`, etc.）

- v2.3.3 (2025-09-07)
  - バイナリ CLI: API 設定に clear-all と show オプションを追加し、config --clear-all で API 設定をリセットしないように変更
  - バイナリ CLI: ``do_translation()`` のエラーハンドリングを改善
  - ライブラリ: translate, languages, usage で各 API 実装をモジュール化

- v2.3.2 (2025-07-07)
  - バイナリ CLI: usage, lang でエンドポイント設定が反映されない問題を修正
  - ライブラリ: request 送信時のクエリのエンコーディングを修正

- v2.3.1 (2025-07-01)
  - バイナリ CLI & ライブラリ: 任意の API エンドポイントを使用可能
  - バイナリ CLI: `set` サブコマンドを `config`, `api`, `cache` サブコマンドに分割
  - テスト: 一部のテストで Python ダミー API サーバを使用
  