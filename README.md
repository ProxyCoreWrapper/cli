# PCW CLI

The PCW CLI manages proxy-core configurations and processes through the PCW
backend's gRPC API over a Unix domain socket.

## Features

- List and read proxy-core configurations.
- Create or update configurations and select their proxy core.
- Start, stop, and restart configured proxy cores.
- Stream stdout and stderr logs from a running proxy core.
- Generate shell completion scripts for Bash, Elvish, Fish, PowerShell, and Zsh.

The protobuf API definitions are in [`proto-files`](https://github.com/ProxyCoreWrapper/proto-files.git).

## Requirements

- A running [PCW backend](https://github.com/ProxyCoreWrapper/backend).
- Rust and Cargo (edition 2024) for building from source.

## Installation

### Arch Linux

Install the packaged CLI from the
[AUR](https://aur.archlinux.org/packages/pcw-cli):

```sh
paru -S pcw-cli
```

The package installs `/usr/bin/pcw-cli` and Bash, Fish, and Zsh completions.

### From source

```sh
git clone https://github.com/ProxyCoreWrapper/cli.git --recurse-submodules
cd cli
cargo build --release
sudo install -Dm755 target/release/cli /usr/local/bin/pcw-cli
```

The build generates completion scripts in `autocompletion`. To install the
supported completions system-wide:

```sh
sudo install -Dm644 autocompletion/pcw-cli.bash \
  /usr/share/bash-completion/completions/pcw-cli
sudo install -Dm644 autocompletion/pcw-cli.fish \
  /usr/share/fish/vendor_completions.d/pcw-cli.fish
sudo install -Dm644 autocompletion/_pcw-cli \
  /usr/share/zsh/site-functions/_pcw-cli
```

The CLI connects to the backend at `/run/pcw/socket` by default.

## Configuration

The CLI reads configuration from the per-user file
`~/.config/pcw-cli.toml`. If that file does not exist, it falls back to
`/etc/pcw/cli-config.toml`. If neither file exists, built-in defaults are used.

The configuration file can specify the backend's Unix domain socket:

```toml
[server]
uds_path = "/run/pcw/socket"
```

## Usage

List available configurations:

```sh
pcw-cli configs list
```

Read a configuration:

```sh
pcw-cli configs get CONFIG_ID
```

Create or update a configuration by reading its contents from a file:

```sh
pcw-cli configs edit CONFIG_ID --new-config /path/to/config.json --core-tag sing-box
```

Use `-` to read the configuration from standard input:

```sh
cat config.json | pcw-cli configs edit CONFIG_ID --new-config - --core-tag sing-box
```

Manage a configuration:

```sh
pcw-cli management start CONFIG_ID
pcw-cli management restart CONFIG_ID
pcw-cli management stop CONFIG_ID
pcw-cli management logs CONFIG_ID
```

Run `pcw-cli --help` or `pcw-cli SECTION --help` for the complete command
reference.

## Shell completions

Completion scripts for Bash, Elvish, Fish, PowerShell, and Zsh are generated in
the `autocompletion` directory when Cargo builds the project. The AUR package
installs the Bash, Fish, and Zsh scripts automatically. For a source
installation, use the commands above or install the generated script for your
shell according to its documentation.

## Development

Build and check the project with:

```sh
cargo check
cargo build --release
```

The generated gRPC Rust code is created by `build.rs` from the protobuf files
when Cargo builds the project.
