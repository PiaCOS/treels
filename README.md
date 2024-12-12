# TREELS

Treels is a recursive version of the famous `ls` command.
**tree + ls = treels**

## Command

```sh
Display your files in a recursive manner

Usage: treels [OPTIONS]

Options:
  -d, --depth <DEPTH>  Depth of the tree [default: 2]
  -p, --path <PATH>    Path to grow your tree [default: .]
  -i, --include-dots   Include dotfiles in the display
  -h, --help           Print help
  -V, --version        Print version
```

## Examples

Get all files in the current folder (without dotfiles) and with a depth of 1:

```sh
treels -d 1
```

Get all files in the parent folder (with dotfiles) with the default depth of 2:

```sh
treels -i -p ../ 
```

## Installation

You can install `treels` in a few different ways:

### Using Cargo (Recommended)

If you have Rust installed, you can easily install `treels` via Cargo:

```sh
cargo install treels
```

### From Precompiled Binaries

If you don’t have Rust installed, you can download precompiled binaries from the Releases page:

- Find the binary for your operating system.
- Download it and place it somewhere in your PATH.

Example on MacOS:
```sh
mv treels-linux /usr/local/bin/treels
chmod +x /usr/local/bin/treels
```

### Building from source

If you prefer, you can clone this repository and build treels manually:

```sh
git clone https://github.com/your-repo/treels.git
cd treels
cargo build --release
cp target/release/treels /usr/local/bin/treels
```



## License

This project is licensed under the [MIT License](LICENSE.md).