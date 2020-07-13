# _improv_

Rust actor microframework.

Improv is an extremely light library (<1k LOC) that sits on top of Tokio and
improves the general Actix-style API by trimming it down to first principles.
Cheers, Nikolay.

## Usage

The current version of Improv is **1.0.0-beta.1**.

To use it, just add it to your Cargo.toml:

```toml
[dependencies]
improv = "1.0.0-beta.1"
```

## Goals

* Improve the Rust actors story by removing the complexity.

* Provide just enough code to enable message-passing as an architecture, but no
  more.

## License

Improv is dual licensed under the Unlicense and Zero-Clause BSD licenses.
