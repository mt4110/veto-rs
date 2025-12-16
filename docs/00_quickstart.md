# 00 Quickstart

Purpose: 3分で導入して「動いた」を確認する。

- Status: Partial Verified
- Last verified: 2025-12-17 (CLI help verified)

## Prerequisites
- Rust 1.75+ toolchain included

## Installation

開発中のため、ソースコードからインストールします。

```bash
# プロジェクトルートで実行
cargo install --path crates/veri-cli --locked
```

インストール後、ヘルプが表示できるか確認します。
```bash
veri --help
```

## Basic Usage

### 1. 動作確認 (Veri Doctor)
環境が正しくセットアップされているか確認します。

```bash
veri doctor
```

### 2. 最小設定の作成
デフォルトでも動作しますが、設定ファイル `veri.toml` を置くことで挙動を制御できます。

```toml
# veri.toml
[output]
format = "text"
fail_on = "high"
```

### 3. スキャンの実行
```bash
# デフォルト（Gitステージング済みファイルのみ対象）
veri scan

# 作業ディレクトリ全体を対象にする場合
veri scan --scope worktree
```

## Common Pitfalls

- **`veri scan` が何も出力しない**: デフォルトは `staged` (コミット予定のファイル) のみが対象です。git add していないファイルはスキャンされません。`--scope worktree` を試してください。
- **設定ファイルが読み込まれない**: カレントディレクトリにある `veri.toml` のみが読み込まれます。
