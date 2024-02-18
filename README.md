# `rfind`

`rfind` is a Rust-based command-line tool designed to mimic the functionality of the GNU `find` command. It allows users to search for files in a directory hierarchy based on a range of criteria such as name patterns, file types, size, and directory depth.

## Features

- **Flexible File Searching**: Search files by name using regular expressions.
- **File Type Filtering**: Narrow your search to files, directories, or symbolic links using the `-t` or `--type` flag.
- **Depth Control**: Specify the minimum (`--min-depth`) and maximum (`--max-depth`) depth of directory traversal to refine your search.
- **Size Criteria**: Find files by size with the `-s` or `--size` flag, using kibibytes (KiB) as the unit and allowing for greater or lesser size comparisons.
- **Customizable Search Paths**: Define where `rfind` begins its search, with support for multiple paths.

## Available Flags

- `-t`, `--type [f|d|l]`: Filter search results by file type. Possible values: `f` for files, `d` for directories, and `l` for symlinks.
- `-n`, `--name <PATTERN>`: Filter files by regex pattern applied to their names.
- `--max-depth <DEPTH>`: Limit the search to a maximum depth relative to each starting point.
- `--min-depth <DEPTH>`: Exclude directories and files above a certain depth from the starting points.
- `-s`, `--size <+|-SIZE>`: Filter files by size. The default option find files larger than the specified size in KiB. Prefix input with `-` to search for smaller sized files.

## Understanding Pattern Matching in `rfind`

`rfind` incorporates a feature for filtering search results based on file name patterns. This functionality leverages Rust's regex engine to match file names against specified patterns. To bridge the gap between common shell glob patterns and Rust's regex syntax, I've included a specialized translator.

#### How It Works

- **Asterisks (`*`)** are translated into `.*`, matching any sequence of characters.
- **Question marks (`?`)** become `.`, matching any single character.
- **Special characters** like `.` `(` `)` `{` `}` `[` `]` `+` `|` `^` `$` `\\` are escaped, ensuring they match literally in file names.

#### Example Usage

To find all `.txt` files, you might use a glob pattern like `*.txt`. `rfind` automatically converts this into the regex pattern `^.*\.txt` for matching.

```bash
# Using glob pattern for finding .txt files
./target/release/rfind . -n "*.txt"
```

#### Limitations

While `glob_to_regex` significantly enhances `rfind`'s usability, it has limitations:

1. **Simple Patterns**: It's designed for basic shell glob patterns and may not fully support advanced globbing features such as negated character classes or brace expansion.
2. **Special Characters**: Literal instances of special regex characters in glob patterns need manual escaping.
3. **Performance**: Regex patterns, especially those with many wildcards, can be less efficient than simple string matches.

## Getting Started

### Prerequisites

- Rust programming language (latest stable version recommended).
- Cargo (Rust's package manager and build system).

### Installation

1. Clone the repository:

```bash
git clone git@github.com:renatodinizc/rfind.git
cd rfind
```

2. Build the project

```bash
cargo build --release
```

3. The executable will be available in `./target/release`

### Usage

To use rfind, run it from the command line, specifying the search criteria and paths. Here are some examples:

```bash
# Find all files in the current directory
./target/release/rfind .

# Find all directories named "src"
./target/release/rfind . -t d -n "^src$"

# Find all .txt files
./target/release/rfind . -n "\\.txt$"

```

### Running Tests
To run the test suite, execute the following command:

```bash
cargo test
```

### Contributing
Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest features.

### License
This project is licensed under the MIT License - see the LICENSE file for details.
