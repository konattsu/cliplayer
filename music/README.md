# musictl - Music Control CLI

YouTubeの音楽配信のタイムスタンプを管理するためのCLIツール

## コマンド一覧

引数等の詳細は`--help`.

```txt
Usage: musictl [OPTIONS] <COMMAND>

Commands:
  add     Add new music entries to the library using one or more input files
  update  Update the existing music library based on its current contents
  sync    Synchronize the library with YouTube using the current library state
  util    Run utility commands that are outside the core music‑library workflows
  help    Print this message or the help of the given subcommand(s)

Options:
      --file-tracing-level <LEVEL>    Tracing level for file operations
      --stdout-tracing-level <LEVEL>  Tracing level for stdout output [default: info]
  -q, --quiet                         If set, suppress stdout tracing output
  -h, --help                          Print help
  -V, --version                       Print version
```

## データ形式

[`README.md`](./data/README.md)を参照
