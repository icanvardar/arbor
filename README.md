# Arbor

**Arbor** is a command-line application written in Rust, designed to provide quick, trie-based autocomplete suggestions. Arbor lets users enter words interactively and suggests completions based on the input prefix.

## Interactive Mode

In interactive mode, Arbor will prompt you to enter words or prefixes. Based on the entered text, it will provide autocomplete suggestions.

1. **Adding Words**: Type a word and press Enter to add it to the trie.
2. **Autocomplete Suggestions**: Type a prefix and press Enter to see a list of words that match the prefix.

### Example:
[![example-recording](https://asciinema.org/a/z5ytDJVDomEmKiu3Nezvkm8Zx.svg)](https://asciinema.org/a/z5ytDJVDomEmKiu3Nezvkm8Zx)

## Features

- **Interactive Autocomplete** - Provides word suggestions based on prefixes entered by the user.
- **Efficient and Lightweight** - Built with Rust for high performance and low memory usage.
- **Easy Installation** - Can be installed directly via Cargo.

## Installation

You can install Arbor using Cargo:

```bash
cargo install arbor
```

## Usage

After installation, simply run `arbor` in your terminal:

```bash
arbor
```

### Command-Line Options

- **`-l`, `--language <LANGUAGE>`**: Specifies the language for suggestions (e.g., `en-US`).
- **`-t`, `--thread-count <THREAD_COUNT>`**: Sets the number of threads for processing (e.g., `4`).
- **`-m`, `--max-suggestion <MAX_SUGGESTION>`**: Limits the number of suggestions returned for a prefix (e.g., `5`).
- **`-b`, `--backup`**: Enables backup mode. When this flag is set, you must also specify the `--output` option.
- **`-o`, `--output <FILE>`**: Specifies the file path for saving backup suggestions (only applicable if `--backup` is enabled).

To exit the application, you can use `Ctrl+C` or `Esc`.

## Contributing

Contributions are welcome! Feel free to fork the repository, open issues, or submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
