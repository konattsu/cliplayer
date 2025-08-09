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
  dev
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
  new     Validate new music data input files
  update  Validate existing music data input files
  help    Print this message or the help of the given subcommand(s)
```

```txt
$ musictl dev --help

Usage: musictl dev <COMMAND>

Commands:
  generate-artist  Generate artist-related data
  duplicate-ids    Check for duplicate video IDs in the input
  merge-files      Merge multiple files containing input music data into a single file
  help             Print this message or the help of the given subcommand(s)
```
