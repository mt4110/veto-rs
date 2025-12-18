# veto-rs

[English](README_EN.md) | **日本語**

**開発者とCIのためのローカル検証ゲート**: 事故が起きる *前* に阻止します。

## 概要
`veto` は、リポジトリ（またはステージングされた差分）に対して一連の **チェック** を実行する Rust 製 CLI ツールです。

- 人間に読みやすい出力（デフォルトで安全）
- 機械可読な JSON 出力
- 必要に応じてコミットや CI をブロックするための非ゼロ終了コード

このリポジトリは小さなワークスペースとして構成されています：
- `veto-core`  : チェックのフレームワーク + レポートモデル（CLIなし、IO仮定なし）
- `veto-config`: 設定のパース + デフォルト値
- `veto-cli`   : ユーザー向け CLI（サブコマンド、フォーマット、終了コード）
- `xtask`      : リリース/開発ヘルパー（オプション）

## 前提条件 (Prerequisites)

マルチプラットフォームでの動作と再現性を保証するため、開発には **Nix** の使用を強く推奨します。

```bash
# direnvを使用する場合（推奨）
direnv allow

# または明示的にシェルに入る場合
nix develop
```

`cargo` コマンドを実行する前に、必ず上記のいずれかで Nix 環境に入ってください。

## クイックスタート
```bash
cargo run -p veto-cli -- scan
```

JSON 出力:
```bash
cargo run -p veto-cli -- scan --format json
```

## 設定 (Config)
コピーして使用します：
- `config/veto.toml.example` → `veto.toml`

## オプションのインフラ`
- `infra/postgres/docker-compose.yml` (監査ログや共有状態が必要な場合のみ)

## ロードマップ (High level)
- Entropy Guard (ステージングされた差分からの秘密情報検出)
- 依存関係チェック (Cargo.lock / npm lock / OSV)
- 署名検証 (タグ / コミット)
- ビルド再現性チェック (Nix 指向)

参照: `docs/ROADMAP.md`
