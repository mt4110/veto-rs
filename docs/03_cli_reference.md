# 03 CLI Reference

Purpose: コマンドライン引数とサブコマンドの詳細（`--help` を一次ソースとして整理）。

- Status: Partial Verified (help verified; behavior pending for checks)
- Last verified: 2025-12-17 (`veri --help`, `veri scan --help`, `veri doctor --help`)

## Overview
`veri` はローカル実行（pre-commit / pre-push / 手動）やCIで使える検証CLIです。

> [!NOTE]
> v0.0.0 時点では `scan` はプレースホルダーチェック（常にパス）を実行します。
> そのため exit code の “しきい値で落ちる挙動” は、実チェック実装後に Verified 化します。

## Global options

- `--repo <REPO>`
  - スキャン対象のリポジトリルート（デフォルト: カレントディレクトリ）
- `--config <CONFIG>`
  - 設定ファイル `veri.toml` のパスを明示指定
  - 指定がない場合は **実行時カレントディレクトリ**の `veri.toml` を探索します（存在すれば読む）
- `-h, --help`
  - ヘルプ表示: そのコマンドの使い方を表示
- `-V, --version`
  - バージョン表示: 現在のバージョンを表示（例: `veri 0.1.0`）

## Subcommands

### `veri doctor`
環境情報と基本診断を表示します。

```bash
veri doctor
```

**出力例（概要）**:
- `repo_root`: スキャン対象のルートパス
- `config`: 読み込まれた設定（custom path / default file / none）
- `rust`: Rustツールチェーンの状態（表示できる場合）

### `veri scan`
チェックを実行し、結果を text / json で出力します。

**Options:**

- `--format <text|json>`
  - 出力形式。`veri.toml` の `[output].format` を上書きします。
- `--scope <staged|worktree|repo>`
  - スキャン範囲。`veri.toml` の `[scope].mode` を上書きします。

**Examples:**

```bash
# デフォルト（configがあれば読み、なければデフォルト）
veri scan

# JSON出力（CIなどで利用）
veri scan --format json

# 範囲を変更（作業中の全変更をスキャン）
veri scan --scope worktree
```

## Exit codes

- `0`: findings が無い、または `fail_on` しきい値未満
- `1`: しきい値以上の severity が存在する（Pending: 実チェック実装後に検証可能）
