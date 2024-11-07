# DirWalker

<div align="center">

<br>
<img src="./src/components/assets/DirWalker.svg" width="130">

<br>

[![Auto Build](https://github.com/fksms/DirWalker/actions/workflows/auto-build.yml/badge.svg)](https://github.com/fksms/DirWalker/actions/workflows/auto-build.yml)

各ディレクトリの使用率をサンバースト図を用いて可視化するツールです。<br>
スキャンを行うディレクトリやスキャンから除外するディレクトリの設定が可能です。<br>
フレームワークはTauriを利用しており、Linux、macOS、Windowsで動作します。<br>
フロントエンドの処理はVue3 + Javascript、バックエンドの処理はRustで書かれています。<br>

<br>
<img src="./images/ScreenRecording.gif" width="550">
<br>
<br>
<br>

</div>

## Screenshots

<div align="center">
<img src="./images/Screenshot_1.png" width="600">
<img src="./images/Screenshot_2.png" width="600">
<img src="./images/Screenshot_3.png" width="600">
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
<br>

Windowsの場合は以下からインストールも可能です。<br>

<a href="https://apps.microsoft.com/detail/9nxz7km9m483">
	<img src="https://get.microsoft.com/images/ja%20dark.svg" width="200"/>
</a>

<br>
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

rustup-initとnodebrewのセットアップ
```
rustup-init
nodebrew setup
```

node.js(npm)の安定版をインストール
```
nodebrew install stable
nodebrew use stable
```

tauri-cliをインストール（v1を利用する）
```
cargo install tauri-cli --version "^1"
```

作業ディレクトリに移動
```
cd DirWalker
```

必要なパッケージをインストール（node_modulesが作成される）
```
npm install
```

Build & Preview（Debug）
```
cargo tauri dev
```

Build（Release）
```
cargo tauri build
```

Build（Debug）
```
cargo tauri build --debug
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