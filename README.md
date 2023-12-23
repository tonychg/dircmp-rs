# dircmp-rs

**dircmp-rs** is a simple command-line tool written in Rust for comparing directory structures.
It helps you identify files that exist in one directory but not in another, making it useful for tasks such as validating backups, syncing folders, or general directory comparison.

## Features

- **Indexing:** Efficiently indexes the contents of source and target directories.
- **Exclusion:** Supports excluding specific files or directories during the comparison.
- **Parallel Processing:** Utilizes parallel processing with Rayon to speed up the comparison process.
- **Logging:** Integrates logging using the `env_logger` crate for informative messages.
- **Print Option:** Provides an option to print the differences to the console.

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/tonychg/dircmp-rs.git
cd dircmp-rs
cargo build --release
```

## Usage

```bash
dircmp-rs <source_directory> <target_directory> [--print] [--exclude <excluded_item> ...]
```

- `--print`: Prints the differences to the console.
- `--exclude`: Exclude specific files or directories during the comparison.

## Examples

```bash
# Compare two directories, print differences
dircmp-rs /path/to/source /path/to/target --print

# Compare two directories, exclude specific files
dircmp-rs /path/to/source /path/to/target --exclude file1.txt file2.txt
```

## Development

Contributions and bug reports are welcome! Feel free to submit issues or create pull requests on the [GitHub repository](https://github.com/tonychg/dircmp-rs).

## License

This project is licensed under the ISC License - see the [LICENSE](LICENSE) file for details.
