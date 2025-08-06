# musictl - Music Control CLI

YouTubeの音楽配信のタイムスタンプを管理するためのCLIツール

## コマンド一覧

引数等の詳細は`--help`.

```txt
$ musictl --help

Usage: musictl <COMMAND>

Commands:
  apply
  validate
  artist
  help      Print this message or the help of the given subcommand(s)
```

```txt
$ musictl apply --help

Usage: musictl apply <COMMAND>

Commands:
  new     Apply new music data from input files
  update  Update existing music data from input files
  sync    Synchronize music data with the existing music directory using the Web API
  help    Print this message or the help of the given subcommand(s)
```

```txt
$ musictl validate --help

Usage: musictl validate <COMMAND>

Commands:
  new-input     Validate new music data input files
  update-input  Validate existing music data input files
  duplicate     Check for duplicate video IDs in the input
  help          Print this message or the help of the given subcommand(s)
```

```txt
$ musictl artist --help

Usage: musictl artist <COMMAND>

Commands:
  generate  Generate artist-related data
  help      Print this message or the help of the given subcommand(s)
```
