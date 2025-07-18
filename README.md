# MatchBox

## 概要

コマンドをカテゴリ別に保存して呼び出せる CLI ツールです。
プログラミング中にメモ帳や別アプリを起動せずに、CLI 上で使用することができます。

## 使用用途

- npm pip cargo 等の様々なのコマンドをカテゴリ別に登録できる。
- 登録したコマンドを素早く呼び出し、クリップボードにコピーできる。

## インストール

```bash
git clone https://github.com/yukisakura001/mb
cd mb
cargo build --release
```

ビルド後、実行可能ファイルを PATH に追加するか、エイリアスを設定してください：

```bash
# エイリアスの例
alias mb="cargo run --"
```

## コマンド一覧

### 基本コマンド

- **mb init**

  - 初期化コマンド。初期状態では Match Box のコマンドが登録されている。

- **mb run**

  - コマンド一覧を呼び出すコマンド。カテゴリ＞コマンド の順番で呼び出す事ができる。

- **mb cat <カテゴリ名>**
  - カテゴリを直接指定してコマンド一覧を出すコマンド。mb run と比べて、コマンドを選択する無駄を減らすことができる。

### カテゴリ・コマンド管理

- **mb newcmd**

  - 既存のカテゴリにコマンドを登録するコマンド。
  - コマンド登録時に、実行文字列を空欄にすることでコマンド名と同じテキストを登録できる。

- **mb newcat**

  - カテゴリを新規作成するコマンド。作成と同時にコマンドを一つ登録する必要がある。

- **mb delcat**

  - カテゴリを削除するコマンド。

- **mb delcmd**
  - カテゴリからコマンドを削除するコマンド。

### 検索・その他

- **mb find <検索文字列> [--n] [--d] [--c]**

  - コマンドを検索するコマンド。
  - `--n`: コマンド名から検索
  - `--d`: 説明から検索
  - `--c`: 実行文字列から検索
  - フラグを指定しない場合、全ての項目から検索

- **mb open**

  - config.json ファイルを既定のエディタで開く。

- **mb ver**
  - バージョンをチェックする。

## 使用例

### 初期化

```bash
mb init
```

### カテゴリの作成

```bash
mb newcat
# カテゴリ名: docker
# コマンド名: ps
# 実行文字列: docker ps -a
# 説明: "全てのコンテナを表示"
```

### コマンドの追加

```bash
mb newcmd
# カテゴリ: dockerを選択
# コマンド名: build
# 実行文字列: docker build -t myapp .
# 説明: イメージをビルド
```

### コマンドの実行

```bash
mb run
# カテゴリを選択 → コマンドを選択 → クリップボードにコピー

mb cat docker
# dockerカテゴリのコマンド一覧から選択
```

### 検索

```bash
mb find build
# "build"を含むコマンドを検索

mb find docker --c
# 実行文字列に"docker"を含むコマンドを検索
```

## 設定ファイル

設定は`config.json`に保存されます。JSON ファイルを直接編集することも可能です：

```json
[
  {
    "name": "docker",
    "commands": [
      {
        "name": "ps",
        "run": "docker ps -a",
        "description": "全てのコンテナを表示"
      },
      {
        "name": "build",
        "run": "docker build -t myapp .",
        "description": "イメージをビルド"
      }
    ]
  }
]
```

## 依存関係

- `arboard` - クリップボード操作
- `inquire` - インタラクティブなプロンプト
- `serde` - JSON シリアライゼーション

## 動作環境

- Windows
- macOS
- Linux

## 貢献

プルリクエストやイシューの報告を歓迎します。
