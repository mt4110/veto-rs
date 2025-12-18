# 00 Quickstart

Purpose: 3分で導入して「動いた」を確認する。

- Status: Verified
- Last verified: 2025-12-18 (Entropy Guard detected, Veto Doctor verified)

## Prerequisites
- Rust 1.75+ toolchain included

## Installation

開発中のため、ソースコードからインストールします。

```bash
# プロジェクトルートで実行
cargo install --path crates/veto-cli --locked
```

インストール後、ヘルプが表示できるか確認します。
```bash
veto --help
```

## Basic Usage

### 1. 動作確認 (Veto Doctor)
環境が正しくセットアップされているか確認します。

```bash
veto doctor
```

### 2. 最小設定の作成
デフォルトでも動作しますが、設定ファイル `veto.toml` を置くことで挙動を制御できます。

```toml
# veto.toml
[output]
format = "text"
fail_on = "high"
```

### 3. スキャンの実行
```bash
# デフォルト（Gitステージング済みファイルのみ対象）
veto scan

# 作業ディレクトリ全体を対象にする場合
veto scan --scope worktree
```

### 4. 検出例 (Entropy Guard)
秘密情報のような高エントロピー文字列が含まれている場合、`High-entropy token detected` として報告され、終了コード 1 で失敗します。

```text
- [HIGH] High-entropy token detected @ src/secrets.rs:10
  Possible secret detected (entropy: 4.88, len: 40). Content: sk_l...8d9a
```
(生のトークン値はマスクされ、ログには残りません)

## Common Pitfalls

- **`veto scan` が何も出力しない**: デフォルトは `staged` (コミット予定のファイル) のみが対象です。git add していないファイルはスキャンされません。`--scope worktree` を試してください。
- **設定ファイルが読み込まれない**: カレントディレクトリにある `veto.toml` のみが読み込まれます。
