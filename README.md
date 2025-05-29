# Pinocchio Template

A simple Solana program template built with the Pinocchio framework for high-performance program development.

## Features

- 🚀 **High Performance**: Built with Pinocchio - Low CU BABY!
- 🧪 **Testing Ready**: Complete test suite using `solana-program-test` - the best balance between control and overhead
- 📦 **Modular Design**: Clean separation between program, SDK, CLI, and tests
- 🔧 **Developer Friendly**: No magic black box - all in simple Rust!

## Installation

```bash
git clone <your-repo-url>
cd pinocchio-template
./test.sh
```

## Design Decisions

Inspired by: https://github.com/Nagaprasadvr/solana-pinocchio-starter

- **No Direct Dependencies**: No crate should use `pinocchio-template-example-program` directly - the SDK forwards all important exports
- **Zero-Copy Performance**: Uses Pinocchio for minimal runtime overhead
- **Comprehensive Testing**: Uses Solana Program Test for integration tests with realistic program interactions. Local validator and other testing frameworks did not meet our needs.
- **Workspace Structure**: Organized as a Cargo workspace for better dependency management

## Contributing

Feel free to make PRs to make this template better!

1. Fork the repository
2. Make your changes
3. Run `./test.sh` to ensure tests pass
4. Submit a pull request

## License

MIT License - see [LICENSE.md](LICENSE.md) file for details
