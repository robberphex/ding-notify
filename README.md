<h1 align="center">ding-notify</h1>
<div align="center">
  <strong>Send markdown to dingtalk, for message with more context infomation.</strong>
</div>

## Install

```sh
$ git clone https://github.com/robberphex/ding-notify.git
$ cargo build --release
# add ./target/release/ding-notify to PATH.
```

## Usage

```
ding-notify 0.1.0

USAGE:
    ding-notify [OPTIONS] <MARKDOWN_PATH>

ARGS:
    <MARKDOWN_PATH>    path to markdown file

OPTIONS:
    -h, --help                               Print help information
    -m, --markdonw-title <MARKDONW_TITLE>
    -V, --version                            Print version information
```

## Example

```sh
./target/debug/ding-notify ./NOTIFY.md --markdonw-title markdown_title
```

## Example of .dingtalk-token.json

```bash
$ cat ~/.dingtalk-token.json
{
	"access_token": "54b2dc82dd9b7bea1xxxxx",
	"sec_token": "SEC450fca6691854677bxxxxx"
}
```

## Contribute

Contributions are welcome. Please open up an issue or create PR if you would like to help out or have new idea.

## License

Licensed under Apache License Version 2.0
