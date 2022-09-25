# headj

A utility that converts input JSON arrays into valid JSON that contains only a subset of the elements

[![PyPI](https://img.shields.io/pypi/v/headj?style=flat-square)](https://pypi.org/project/headj)
[![PyPI - Implementation](https://img.shields.io/pypi/implementation/headj?style=flat-square)](https://pypi.org/project/headj)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/headj?style=flat-square)](https://pypi.org/project/headj)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/headj?style=flat-square)](https://pypistats.org/packages/headj)
[![PyPI - License](https://img.shields.io/pypi/l/headj?style=flat-square)](https://opensource.org/licenses/MIT)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)

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

One very large caveat is: **It freely discards JSON that surrounds the array of interest. So, if you have
a complex JSON object with a huge array in it, you will get a JSON file back that only includes the
(reduced) array & whatever JSON structure it was found in. Everything else will be elided.**

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

### With `pipx`

```shell
pipx install headj
```

## Usage

```
usage: headj [-h] [-q] [-k KEYS] [-c COUNT] [-s SKIP] [-f] [-o OUTPUT] [-d] [-t] [infile]

positional arguments:
  infile                The JSON file to read from. If none is specified, reads from
                        Standard Input

optional arguments:
  -h, --help            show this help message and exit
  -q, --quiet           Don't print any status, diagnostic or error messages
  -k KEYS, --key KEYS   The JSON key of the array to copy from. If none specified, treat
                        the input JSON as an array.
  -c COUNT, --count COUNT
                        Number of elements to copy to the output (default: 100)
  -s SKIP, --skip SKIP  Number of elements to skip before copying (default: 0)
  -f, --format          Nicely format the output JSON with indentation & newlines.
  -o OUTPUT, --output OUTPUT
                        File to write the JSON results to (default: Standard Output)
  -d, --debug           Activate extra debugging output
  -t, --trace           Show a stack trace for exceptions

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
* The deletion of all JSON elements except the ones of interest is "bad". It needs to be fixed (or at least optional).
* The error messages can be comically unhelpful.
* The examples could be improved a trifle.

## License

MIT
