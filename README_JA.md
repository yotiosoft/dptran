# dptran

日本語 | [English](README.md)

![Crates.ioバージョン](https://img.shields.io/crates/v/dptran)
[[CI](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/yotiosoft/dptran/actions/workflows/rust.yml)
[ライセンス: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[[License: Apache-2.0](https://img.shields.io/badge/License-Apache 2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**dptran**は、Rust で作られた DeepL API を使用するためのコマンドライン・ツールおよびライブラリです。

## 特徴

### バイナリCLI

- CLI、対話型入力、パイプライン、またはファイル/エディタ入力を介して翻訳
- 複数行入力、ファイルへの出力、改行の削除
- DeepL API Free / Pro のサポート
- 言語コードの検索と文字使用のトラッキング
- 結果のキャッシュ

### Library

- DeepL 翻訳用 API クライアント
- 言語コードと usage（残り翻訳可能文字数）の取得

## インストール

### バイナリー・クレート
```bash
cargo install dptran
```

### ライブラリクレート
```bash
cargo add dptran --no-default-features
```
## 基本的な使い方
### APIキーの設定
```bash
dptran api --api-key-free [あなたのAPIキー].
# or set env: export DPTRAN_DEEPL_API_KEY=[あなたのAPIキー].
```

### 翻訳する
```bash
# 簡単な翻訳
dptran Hello
# ターゲット言語で翻訳する
dptran -t JA Hello
# ソース言語で翻訳する
dptran -f EN -t JA Hello
# 対話的に翻訳する
dptran
> Hello
# ファイルからの翻訳
dptran -i text.txt
# パイプラインで翻訳する
echo "Hello" | dptran -t JA
# 改行を削除して翻訳する
dptran -r "Hello\nWorld"
# エディタから翻訳する（vimやemacsなど。事前の設定が必要）
dptran -e
```

### オプション
- -t 翻訳先言語を指定する
- -f [LANG] 翻訳元言語を指定する
- -o [FILE] ファイルに出力
- -r 改行を削除
- -u 文字の usage を表示
- list -s / -t 利用可能な言語コードを表示する。

より多くのオプションと詳細な使用法については下記を参照してください：
```bash
dptran -h
```

### 構成
デフォルトのターゲット言語を変更します：

```bash
dptran config --target-lang JA
```
すべての設定をリセットする：

```bash
dptran config --clear-all
```

## 開発
ユニットテストを実行します。
実際の DeepL API キーを必要とするテストを実行するには、環境変数 `DPTRAN_DEEPL_API_KEY` を設定します：

```bash
export DPTRAN_DEEPL_API_KEY=[APIキー]
cargo test -- --test-threads=1
```

ダミーのAPIサーバーが稼動している必要があるものもあります。  
ダミーサーバーはデフォルトで`http://localhost:8000/`で実行されます。

```bash
pip3 install -r requirements.txt
uvicorn dummy-api-server:app --reload
```

## ドキュメント
crate ページ：https://crates.io/crates/dptran

ライブラリ ドキュメント：https://docs.rs/dptran

## ライセンス
下記のライセンスのいずれかで使用できます。

- MITライセンス
- Apacheライセンス2.0
