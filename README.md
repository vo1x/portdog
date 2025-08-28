# portdog

`portdog` is a cross-platform process management utility built with Rust that helps you manage network port usage. 

### Features

- Find processes owning specific ports
- Kill processes using designated ports
- Discover available ports within a specified range (WIP)
- Support for filtering by TCP and UDP protocols

## Installation

### Pre-built Binaries

You can download the latest pre-built binary for your platform from the [GitHub Releases](https://github.com/vo1x/portdog/releases) page.

#### Installation

For macOS and Linux, you can use the installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/vo1x/portdog/main/install.sh | bash
```

Or with a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/vo1x/portdog/main/install.sh | VERSION=v0.1.0 bash
```

#### Manual Installation

##### macOS
```bash
# For Intel Macs
tar -xzf portdog-v*-x86_64-apple-darwin.tar.gz
sudo mv portdog-*/portdog /usr/local/bin/

# For Apple Silicon (M1/M2) Macs
tar -xzf portdog-v*-aarch64-apple-darwin.tar.gz
sudo mv portdog-*/portdog /usr/local/bin/
```

##### Linux
```bash
tar -xzf portdog-v*-x86_64-unknown-linux-gnu.tar.gz
sudo mv portdog-*/portdog /usr/local/bin/
```

#### Windows
1. Download the zip file for Windows
2. Extract the contents
3. Add the extracted directory to your PATH or move the executable to a directory in your PATH

### Building from Source

```bash
git clone https://github.com/vo1x/portdog.git
cd portdog
cargo build --release
```
The binary will be available at `target/release/portdog`.

### Usage

#### Build and run (from source)
```bash
# Build in release mode
cargo build --release

# Show help
./target/release/portdog --help
./target/release/portdog help who
./target/release/portdog help kill
```
#### Examples (pre-built binary)

- Find which process(s) owns a port
```bash
# Any protocol (default)
portdog who 3000

# TCP only
portdog who 5432 --proto tcp

# UDP only
portdog who 53 --proto udp
```

- Kill process(s) using a port
```bash
# Graceful stop (SIGTERM on Unix; normal taskkill on Windows)
portdog kill 3000

# Force stop (SIGKILL on Unix; /F on Windows)
portdog kill 8080 --force
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
