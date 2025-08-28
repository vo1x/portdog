# portdog

`portdog` is a cross-platform process management utility built with Rust that helps you manage network port usage. 

### Features

- Find processes owning specific ports
- Kill processes using designated ports
- Discover available ports within a specified range (WIP)
- Support for filtering by TCP and UDP protocols

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
