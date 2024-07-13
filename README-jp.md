# DirWalker

<div align="center">

<br>
<img src="assets/icon/DirWalker.svg" width="130">

<br>各ディレクトリの使用率をサンバースト図を用いて可視化するツールです。
<br>スキャンを行うディレクトリや除外するディレクトリの設定が可能です。
<br>フロントエンドの処理はVue3、バックエンドの処理はRustを使用しています。

<br>
</div>

## Screenshots

<div align="center">
<img src="assets/Screenshot_1.png" width="600">
<img src="assets/Screenshot_2.png" width="600">
<img src="assets/Screenshot_3.png" width="600">
</div>

## Install

[Release](https://github.com/fksms/DirWalker/releases)を確認してください。

- MacOS (Intel)<br>
  `DirWalker_X.X.X_x64.dmg` をダウンロードして展開してください。

- MacOS (Apple Silicon)<br>
  `DirWalker_X.X.X_aarch64.dmg` をダウンロードして展開してください。

- Windows<br>
  `DirWalker_X.X.X_x64-setup.exe` をダウンロードして実行してください。

<br>

## How to build

### GitHub Actions

`All Build Release` を手動で実行することで、MacOS、Windows、Linux用のBuild（Release）が可能で、Build完了後、ダウンロード用URLが発行されます。

`All Build Debug` を手動で実行することで、MacOS、Windows、Linux用のBuild（Debug）が可能で、Build完了後、ダウンロード用URLが発行されます。

<br>

### MacOS

rustup-initとnodebrewをインストール
```
brew install rustup-init
brew install nodebrew
```

node.js(npm)の安定版をインストール
```
nodebrew install stable
```

tauri-cliをインストール
```
cargo install tauri-cli
```

作業ディレクトリに移動
```
cd DirWalker
```

必要なパッケージをインストール（node_modulesが作成される）
```
npm install
```

Build（Debug）
```
cargo tauri dev
```

Build（Release）
```
cargo tauri build
```

<br>

### Windows (GitHub Actions)

`Windows Build Release` を手動で実行することで、Build（Release）が可能で、Build完了後、ダウンロード用URLが発行されます。

`Windows Build Debug` を手動で実行することで、Build（Debug）が可能で、Build完了後、ダウンロード用URLが発行されます。

<br>

## Contribution
Issueへの投稿、PullRequest大歓迎です。
バグや改善点を見つけた場合は、遠慮なくPullRequestを送ってください。
日本語がメインなので、英訳も歓迎です。

<br>

## License
MIT License<br>
Copyright (c) 2024 Shogo Fukushima