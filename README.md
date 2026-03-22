# gh-review-watcher

GitHub上で自分にレビュー依頼された / アサインされたPRをポーリングで監視するTUIアプリ。

## 機能

- **レビュー依頼PR** (`--review-requested=@me`) と **アサインPR** (`--assignee=@me`) を一覧表示
- 新規PR検出時に任意のコマンドを実行（macOS通知、zellijペイン起動など）
- Enter押下で選択PRに対するアクション実行
- 時間ベースのフィルタ（All / 24h / 7d）でアクティブなPRに絞り込み
- リフレッシュ中のインジケーター表示

## キーバインド

| キー | アクション |
|------|-----------|
| `q` | 終了 |
| `j` / `k` | 上下移動 |
| `Enter` | 選択PRのアクション実行 |
| `r` | 手動リフレッシュ |
| `Tab` | フィルタ切り替え (All → 24h → 7d) |

## 設定

`~/.config/gh-review-watcher/config.toml`:

```toml
# ポーリング間隔（秒）
interval = 120

# 新規PR検出時: macOS通知
[[on_new_pr]]
name = "notify"
command = """/Applications/Utilities/Notifier.app/Contents/MacOS/Notifier \
  --type banner --title "PR Review Request" \
  --subtitle "{repo} #{number}" \
  --message "{title} by @{author}" \
  --sound default"""

# 新規PR検出時: zellijでfloatingペインを開いてレビュー開始
[[on_new_pr]]
name = "review-pane"
command = """zellij action new-pane --floating --name "Review: {repo}#{number}" -- \
  claude --dangerously-skip-permissions -p \
  "PR #{number} in {repo} をレビューしてください。URL: {url}"""

# Enter押下時: ブラウザでPRを開く
[on_select]
command = "open {url}"
```

### テンプレート変数

| 変数 | 内容 |
|------|------|
| `{repo}` | リポジトリ名 (owner/repo) |
| `{number}` | PR番号 |
| `{title}` | PRタイトル |
| `{author}` | PR作成者 |
| `{url}` | PRのURL |

## インストール

### Nix Flake

```nix
# flake.nix の inputs に追加
gh-review-watcher = {
  url = "github:EdV4H/gh-review-watcher";
  inputs.nixpkgs.follows = "nixpkgs";
};
```

### Cargo

```bash
cargo install --path .
```

## ビルド

```bash
nix build
# or
cargo build --release
```

## 前提条件

- [GitHub CLI (`gh`)](https://cli.github.com/) がインストール・認証済みであること
