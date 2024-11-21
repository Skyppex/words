# words

`words` does stuff relating to words. Stuff like counting them, checking if they
exist, removing puntuation around them, and turning them into json.

Pro tip: `words` can also work with sentences.
Pro pro tip: `words` cannot work with letters. Use [letters](https://github.com/Skyppex/letters) for that.

## Installation

### Build from source

- Clone the repo
  `git clone https://github.com/Skyppex/words.git`
  `gh repo clone Skyppex/words`

- Build with cargo
  `cargo build --release`

## Usage

Usage: words.exe [OPTIONS]

Options:

- `-s`, `--source` \<SOURCE\> The source file to read from. If not provided, read from stdin
- `-d`, `--destination` \<DESTINATION\> The destination file to write to. If not provided, write to stdout
- `-v`, `--verbose` Enable verbose logging
- `-q`, `--quiet` Suppress all informational output. Errors will still be printed to stderr
- `-f`, `--first` [\<FIRST\>] Get the first n words from the input. (default 1)
- `-l`, `--last` [\<LAST\>] Get the last n words from the input. (default 1)
- `-c`, `--contains` \<CONTAINS\> Filter the input to only include words that contain the given substring
- `-C`, `--case-sensitive` Case-sensitive matching
- `-S`, `--sentence` Work with entire sentences instead of individual words. Sentences are split on periods. The output will always end with a period. Words are split on whitespace
- `-L`, `--list` Print the result as a list separated by newlines
- `-j`, `--json` Print the result as a json list
- `-p`, `--no-punctuation` Remove punctuation from the output
- `-t`, `--trim` Trim whitespace from the output
- `-n`, `--count` Count the number of words in the output
- `-h`, `--help` Print help
- `-V`, `--version` Print version

## Contributing

Issues and PRs are welcome!
