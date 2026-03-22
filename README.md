# gh-review-watcher

GitHub上で自分にレビュー依頼されたPRをポーリングで監視するTUIアプリ。

## 機能

- `gh search prs --review-requested=@me` を定期実行してPR一覧を表示
- 新規PR検出時に任意のコマンドを実行（通知、ペイン起動など）
- Enter押下で選択PRに対するアクション実行

## キーバインド

| キー | アクション |
|------|-----------|
| `q` | 終了 |
| `j` / `k` | 上下移動 |
| `Enter` | 選択PRのアクション実行 |
| `r` | 手動リフレッシュ |

## 設定

`~/.config/gh-review-watcher/config.toml`:

```toml
interval = 120

[[on_new_pr]]
name = "notify"
command = "echo 'New PR: {repo} #{number} - {title} by @{author}'"

[on_select]
command = "open {url}"
```

テンプレート変数: `{repo}`, `{number}`, `{title}`, `{author}`, `{url}`

## ビルド

```bash
nix build
# or
cargo build --release
```
