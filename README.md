# Unicool &emsp; ![Build Status]

[Build Status]: https://travis-ci.org/oknozor/unicool.svg?branch=master

Unicool is a small set of tools to convert non ascci characters in a file or raw input to escaped unicode characters. 

## Run the cli 

```sh
$ git clone https://github.com/oknozor/unicool.git
$ cargo build -p unicool-cli
```

The cli is available in `target/debug/unicool-cli`, use the `--help` flag to see what you can do. 

## Run the web app 

```sh
$ cargo install cargo-web
$ cargo web start -p unicool-web
```

Then browse to `http://[::1]:8000/`