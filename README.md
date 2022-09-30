# headj

A utility that converts input JSON arrays into valid JSON that contains only a subset of the elements

[![crates.io](https://img.shields.io/crates/v/headj.svg)](https://crates.io/crates/headj)
[![docs.rs](https://docs.rs/headj/badge.svg)](https://docs.rs/headj)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Latest version](https://img.shields.io/crates/v/headj.svg)](https://crates.io/crates/headj)
[![All downloads](https://img.shields.io/crates/d/headj.svg)](https://crates.io/crates/headj)
[![Downloads of latest version](https://img.shields.io/crates/dv/headj.svg)](https://crates.io/crates/headj)

## Description

A utility to take enormous JSON files & cut them down to size.

Sometimes one has a JSON file with a very, very large array in it, but you really would like to have a
subset of the data. For example, you have a JSON file representing a DB dump of millions of records &
you'd like to have a workable number of rows to test with, or code with, or just examine.

You _could_ use an editor, but even if the editor can load a file that large, it's likely to be very
unpleasant to use.

You _could_ just use some kind of text processor (like the unix/linux `head` command)
but there are two issues with that:

1. If the file doesn't have newlines, that's not helpful.
2. The text processor will know nothing about JSON, so it will mangle it.

You _could_ write a script/program to do the work for you. I did **that**. Then I decided to package it up, so
you don't have to.

`headj` is a command line utility, similar to the `head` command, for producing a subset of a JSON file that is
itself valid JSON. It allows you to ingest JSON containing a huge JSON array I produce JSON with a manageable JSON
array.

### _THIS IS ALPHA LEVEL SOFTWARE_

It appears to work, but I'm working on it.

One very large caveat is: ~~It freely discards JSON that surrounds the array of interest. So, if you have
a complex JSON object with a huge array in it, you will get a JSON file back that only includes the
(reduced) array & whatever JSON structure it was found in. Everything else will be elided.~~ **(This actually works correctly now)**

For example:

Input

```json
{
  "a": 1,
  "b": [
    1,
    2,
    3,
    4,
    5
  ],
  "c": true
}
```

command: `headj --key 'b' --count 3`

output

```json
{
  "b": [
    1,
    2,
    3
  ]
}
```

## Installation

### With `Cargo`

```shell
cargo install headj
```

## Usage

```
USAGE:
    headj [OPTIONS] [INPUT_FILE]

ARGS:
    <INPUT_FILE>    The JSON file to read from. If none is specified, reads from Standard Input

OPTIONS:
    -c, --count <COUNT>          Number of elements to copy to the output (default: 100) [default:
                                 100]
    -d, --debug                  Activate extra debugging output
    -f, --format-output          Nicely format the output JSON with indentation & newlines
    -h, --help                   Print help information
    -k, --key <KEY>              The JSON key of the array to copy from. If none specified, treat
                                 the input JSON as an array
    -n, --no-context             Output _only_ the target JSON array
    -o, --out-file <OUT_FILE>    File to write the JSON results to (default: Standard Output)
    -q, --quiet                  Don't print any status, diagnostic or error messages
    -s, --skip <SKIP>            Number of elements to skip before copying (default: 0) [default: 0]
    -V, --version                Print version information
```

## Examples

```shell
headj <<- JSON
[1,2,3,4,5]
JSON
# Output: [1, 2, 3, 4, 5]

headj -c 1 <<- JSON
[1,2,3,4,5]
JSON
# Output: [1]

headj -c 1 -s 2 <<- JSON
[1,2,3,4,5]
JSON
# Output: [3]

headj -c 2 -s 2 <<- JSON
[1,2,3,4,5]
JSON
# Output: [3, 4]

headj -k 'foo' <<- JSON
{"foo":[1,2,3,4,5]}
JSON
# Output: {"foo": [1, 2, 3, 4, 5]}

headj -k 'foo' -n <<- JSON
{"foo":[1,2,3,4,5]}
JSON
# Output: [1, 2, 3, 4, 5]

headj -c 2 -s 2 <<- JSON
[1,2,3,4,5]
JSON
# Output: [3, 4]

headj -c 25 -s 2 <<- JSON
[1,2,3,4,5]
JSON
# Output: [3, 4, 5]

headj -c 2 -s 2 <<- JSON
[1,2,3,4,5]
JSON
# Output: [3, 4]

headj -c 2 -s 2 -f <<- JSON
[1,2,3,4,5]
JSON
# Output: [\n     3,\n     4\n]

headj -k 'foo.bar' -c 2 -s 2 -n <<- JSON
{"foo":{"bar":[1,2,3,4,5]}}
JSON
# Output: [3, 4]

headj -k 'foo.bar' -c 2 -s 2 <<- JSON
{"foo":{
"bar":[1,2,3,4,5]}
}
JSON
# Output: {"bar": {"foo": [3, 4]}}

headj -k 'foo' -c 2 -s 2 <<- JSON
{"foo":[1,2,3,4,5]}
JSON
# Output: {"foo": [3, 4]}

headj -k 'foo' -c 2 -s 2 -n <<- JSON
{"foo":[1,2,3,4,5]}
JSON
# Output: [3, 4]

```

## Documentation

* [The FAQ](https://github.com/evanjpw/headj-rs/blob/main/doc/faq.md)

## The TBD

* The `--key` is based, in the most vague sense, on the [JSON Schema](https://json-schema.org) specification.
  It's more of a gesture in the general direction of JSON Schema. The reason that it doesn't use the full
  JSON Schema is, the most natural way to write keys if you don't feel like reading a specification would
  not necessarily start at the root, which would be potentially confusing. Insisting that users begin
  with a '`$`' would likely seem arbitrary & annoying. So, the "just dots & backslashes" implementation seemed
  reasonable.
* ~~The deletion of all JSON elements except the ones of interest is "bad". It needs to be fixed (or at least optional).~~
* The error messages can be comically unhelpful.
* The examples could be improved a trifle.
* The `--format` option currently **DOES NOTHING**. Working on it.

## License

MIT
