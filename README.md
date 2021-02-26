# newsroom

## Installation
In order to use `newsroom`, you have to install `cargo` tool then run the following command :
```sh
cargo build --release
```

If the build fails, make sure to use an up-to-date Rust version.
```sh
rustup update
```

## How use it ?
To launch `newsroom`, you can both use `cargo` by running :
```sh
cargo run --release -- <argument>
```

Or by executing the `newsroom` binary directly :
```sh
./target/release/newsroom <argument>
```

## Usage
``` sh
newsroom 0.1.0

USAGE:
    newsroom [OPTIONS] [source]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --max-page <max-page>     [default: 10]

ARGS:
    <source>         Available sources: hackersnews, producthunt, techmeme, wsj or github-trending

```
