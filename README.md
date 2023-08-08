# webreg (web regex)

CLI tool for testing regexes against web pages.

Test if a list of websites match a given regex

## Installation

```bash
cargo install webreg
```

## Usage

```bash
webreg [OPTIONS] <REGEX>
```

```
Arguments:
<REGEX> A regular expression to match against the site content

Options:
-u, --urls <URLS>       Comma separated list of urls
-i, --file <FILE>       A file containing a list of urls
-c, --case-insensitive  Case insensitive search
-f, --fix-urls          Fix urls that don't start with http:// or https://
-r, --retry             Retry failed urls
-s, --save              Saves the output to the results folder (./results/<regex>)
-h, --help              Print help
```

## Examples

### Basic usage

```bash
webreg -u "https://example.com" "Hello World"
```

This will check if the string "Hello World" is present in the content of https://example.com. If it is, it will print the url to stdout.

### Multiple urls

```bash
webreg -u "https://example.com,https://example.org" "Hello World"
```

### Domains

```bash
webreg -u -f "example.com,example.org" "Hello World"
```

The `-f` flag will fix urls that don't start with http:// or https://

### Case insensitive

```bash
webreg -u -c "https://example.com" "hello world"
```

The `-c` flag will make the search case insensitive.

### File input

```bash
webreg -i urls.txt "Hello World"
```

`urls.txt`:

```bash
https://example.com
https://example.org
```

The `-i` flag will read the urls from a file. The file should contain one url per line. Empty lines will be ignored and whitespace will be trimmed.

### Pipe input

```bash
cat urls.txt | webreg -i "Hello World"
```

`urls.txt`:

```bash
https://example.com
https://example.org
```

### Save the output

```bash
webreg -u -s "https://example.com" "Hello World"
```

The `-s` flag will save the output to the results folder (`./results/<regex>`). This will also output lists urls that couldn't be fetched and urls that didn't match the regex.
