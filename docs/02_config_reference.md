# 02 Config Reference

Purpose: 設定ファイルの全オプションと仕様。

- Status: Partial Verified (Schema verified; Behavior pending)
- Last verified: 2025-12-17 (config load + precedence tested)

## Overview
`veri.toml` をプロジェクトルートに配置することで、スキャンの挙動をカスタマイズできます。

> [!NOTE]
> **Implementation status (v0.0.0)**
> 本ドキュメントは **設定スキーマの仕様** を定義します。allowlist/entropy_guard 等の設定はロードされますが、実際のチェック処理への適用は対応するチェック実装の完了を待って有効になります。

## Precedence
設定の優先順位は以下の通りです（上が優先）：

1. **CLI引数** (例: `--format json`)
2. **設定ファイル** (`veri.toml`)
3. **デフォルト値** (Code内蔵)

## Minimal Config
何も書かなくても動作しますが、よくある設定例です。

```toml
[output]
format = "text"
fail_on = "high"
```

## Full Example
全てのオプションを明示した例です。

```toml
[output]
format = "text"         # "text" | "json"
fail_on = "high"        # "low" | "medium" | "high" | "critical"

[scope]
mode = "staged"         # "staged" | "worktree" | "repo"

[allowlist]
patterns = []           # 許可するパターンのリスト

[entropy_guard]
enabled = true
min_length = 24
threshold = 4.2
ignore_ext = ["png", "jpg", "gif", "mp4", "pdf"]
```

## Reference

### `[output]`
出力形式と終了コードの制御。

- **`format`** (String)
    - デフォルト: `"text"`
    - 許容値: `"text"`, `"json"`
    - 説明: スキャン結果の表示形式。CI等で機械可読が必要な場合は `json` を推奨。
- **`fail_on`** (String)
    - デフォルト: `"high"`
    - 許容値: `"low"`, `"medium"`, `"high"`, `"critical"`
    -説明: 指定した重大度以上のissueが見つかった場合に終了コード 1 を返す。(Pending: チェック実装後に検証可能)

### `[scope]`
スキャン対象の範囲。

- **`mode`** (String)
    - デフォルト: `"staged"`
    - 許容値: `"staged"`, `"worktree"`, `"repo"`
    - 説明:
        - `staged`: Gitのステージングエリアのファイルのみ（コミット前フック用）
        - `worktree`: 作業ディレクトリの変更分
        - `repo`: リポジトリ全域

### `[allowlist]`
誤検知を抑制するための例外設定。

- **`patterns`** (Vec<String>)
    - デフォルト: `[]`
    - 説明: 許可（無視）したいパターンのリスト。
    - Matching: (TBD: exact / substring / regex)

### `[entropy_guard]`
高エントロピー文字列（秘密鍵など）の検出設定。

- **`enabled`** (bool)
    - デフォルト: `true`
    - 説明: エントロピースキャンを有効にするか。
- **`min_length`** (usize)
    - デフォルト: `24`
    - 説明: チェック対象とする最小文字列長。これより短い文字列は無視。
- **`threshold`** (f64)
    - デフォルト: `4.2`
    - 説明: Shannonエントロピーの閾値。これを超えると検出される。
- **`ignore_ext`** (Vec<String>)
    - デフォルト: `["png", "jpg", "gif", "mp4", "pdf"]`
    - 説明: エントロピーチェックから除外する拡張子（バイナリファイル等）。

## Notes
- **ファイルパス**: 設定ファイルはデフォルトで **実行時のカレントディレクトリ** の `veri.toml` を探します。
    - 別のパスを指定したい場合は `--config path/to/veri.toml` を使用してください。
    - `--repo` オプションはスキャン対象のルートを指定するものであり、設定ファイルの探索場所には影響しません（独立しています）。
- **`entropy_guard.threshold`**: 値を下げすぎると誤検知が増えます。通常はデフォルトの `4.2` 〜 `4.5` 程度が推奨です。

## TODO
- [ ] `allowlist.patterns` のマッチング仕様（正規表現対応有無）を確定させる
- [ ] `fail_on` のExit Code挙動を実装後に検証する
