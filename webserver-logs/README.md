# Webserver Log Parsing

_Regex for IP address matching for both versions found courtesy of Google rather than writing from scratch (https://www.shellhacks.com/regex-find-ip-addresses-file-grep/)_

## Scripting

Paste the following command in this directory containing the mock `example.txt`.

```sh
./calculate.sh example.txt
```

## Programming

Chose to do the programming language version in Rust with a little CLI tool.

To run (assuming Cargo/Rust installed), ensure `cd`'d into this directory and passing the relative path to the example file:

```sh
cargo run --release -- --file example.txt
```
