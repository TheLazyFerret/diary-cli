# Diary cli
Small and simple CLI diary. The first time is run each day, it will create a `.txt` file with the date as filename.

## Installation
```bash
git clone https://github.com/TheLazyFerret/diary-cli.git
cd diary-cli
cargo install .
```

## Usage
The first time it is run, will attempt to create a data directory in `$XDG_DATA_HOME/diary-cli`. If the program can´t fetch the environment variable, will fallback to `$HOME/.local/share/diary-cli`.

In order to open the file, it is necessary set the environment variable `$EDITOR` to a valid value.

```bash
Usage: diary-cli [OPTIONS]

Options:
  -d, --debug        Show verbose output (in stderr)
  -l, --list         List all the entries
  -s, --show <SHOW>  Open an specific day
  -h, --help         Print help
  -V, --version      Print version
```

## License
This project is under the [MIT](LICENSE) license.