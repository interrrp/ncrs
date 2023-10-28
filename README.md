# Netcat, but in [Rust 🦀](https://rust-lang.org)

## Installation

Clone this repository, build with [Cargo](https://github.com/rust-lang/cargo),
and install:

```sh
git clone https://github.com/interrrp/ncrs
cd ncrs
cargo install
```

If you have added the Cargo binary path to PATH, you can run `ncrs` via `ncrs`.

## Usage

There are only three command-line arguments at the moment:

| Name             | Description                                 |
| ---------------- | ------------------------------------------- |
| `-h`             | Show the help message.                      |
| `ADDRESS`\*      | The address to connect/bind to (required)   |
| `-s`, `--server` | Run in server mode (listen for connections) |

## Examples

Connect to a TCP server at `localhost:3000`:

```sh
ncrs localhost:3000
```

Start a TCP server at `localhost:3000`:

```sh
ncrs localhost:3000 -s
```
