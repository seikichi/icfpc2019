# Tools

セットアップするにはプロジェクトのルートで `npm` を実行して下さい。このとき以下が必要となります。

- node.js
- Headless Chrome が実行可能な環境

AWS と通信するコマンドの場合は追加で以下のいずれかの設定を行って下さい

- 環境変数 `AWS_ACCESS_KEY_ID` および `AWS_SECRET_ACCESS_KEY` の設定
- 共有認証情報ファイルの作成 (`~/.aws/credentials`) および必要に応じて環境変数 `AWS_PROFILE` の設定

その他:

- Q. なぜ JavaScript なのですか？
- A. https://github.com/GoogleChrome/puppeteer

## submit.sh

現在の解答を提出します:

```sh
> export ICFPC2019_PRIVATE_ID=...
> ./tools/submit.sh
```

## upload.js

解答が正しいかどうかを検証し、過去のスコアより良い成績だった場合に S3 にアップロードします。

```sh
> node tools/upload.js 010 path/to/prob-010.desc path/to/prob-010.sol
solutions/problems/prob-001.desc does not exists, upload the given solution.

> node tools/upload.js 010 path/to/prob-010.desc path/to/new-prob-010.sol
The given solution (10) seems better than old one (13), upload it ...
```

## checker.js

公式の Checker を利用して解答が正しいかどうか検証します。結果を標準出力に出力します。また失敗時はステータスコード 255 です。

```sh
> node tools/checker.js path/to/prob-010.desc path/to/prob-010.sol
{"success":true, "timeunits":585}

> echo $?
0

> node tools/checker.js path/to/prob-010.desc path/to/prob-011.sol
{"success":false, "timeunits":null}

> echo $?
255
```
