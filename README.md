[![Test](https://github.com/bornacvitanic/file_aggregator/actions/workflows/rust.yml/badge.svg)](https://github.com/bornacvitanic/file_aggregator/actions/workflows/rust.yml) [![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/bornacvitanic/file_aggregator/releases)

# File Aggregator

File Aggregator is a utility for aggregating and distributing file contents. It allows you to combine file contents from a specified directory into a single text, which is copied to the clipboard. You can also distribute file contents from the clipboard back to their respective files based on the relative paths.

### Features

- **Aggregate File Contents**: Combine the contents of multiple files from a specified directory into a single text.
- **Distribute File Contents**: Distribute the contents from the clipboard back into their respective files based on relative paths.
- **Clipboard Integration**: Copy the aggregated contents directly to the clipboard for easy sharing.
- **Path and Extension Filtering**: Specify the root path and file extensions to include or exclude certain files.


### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/bornacvitanic/file_aggregator.git
   cd file_aggregator
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

### Usage

```
fileagg [COMMAND]
```

#### Commands

- `aggregate`: Aggregates file contents.
- `distribute`: Distributes file contents.
- `help`: Print this message or the help of the given subcommand(s).

#### Options

- `-p, --path <PATH>`: The path to use for the operation. If not specified, defaults to the current directory.
- `-e, --extensions <EXTENSIONS>`: A comma-separated list of file extensions to include. If not specified, all files are included.

### Examples

1. Aggregate file contents:
   ```sh
   fileagg aggregate --path "/path/to/directory" --extensions "rs,txt,md"
   ```

2. Distribute file contents:
   ```sh
   fileagg distribute --path "/path/to/directory"
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgements

- [clap](https://docs.rs/clap/4.5.9/clap/) - Library for command-line interface (CLI) parsing
- [walkdir](https://docs.rs/walkdir/2.5.0/walkdir/) - Library for recursive directory traversal
- [copypasta](https://docs.rs/copypasta/0.10.1/copypasta/) - Library for clipboard access

## Contact

- **Email**: [borna.cvitanic@gmail.com](mailto:borna.cvitanic@gmail.com)
- **GitHub Issues**: [GitHub Issues Page](https://github.com/bornacvitanic/file_aggregator/issues)