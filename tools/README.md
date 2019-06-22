# Tools

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

セットアップするにはプロジェクトのルートで `npm` を実行して下さい。このとき

- node.js
- Headless Chrome が実行可能な環境
