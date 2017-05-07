bcmp-cli
========

[![Build Status](https://travis-ci.org/haxelion/bcmp-cli.svg?branch=master)](https://travis-ci.org/haxelion/bcmp-cli)

`bcmp-cli` offers a CLI interface to the [`bcmp`](https://github.com/haxelion/bcmp) crate to compare 
binary files.

Longest Common Substrings
-------------------------

```
bcmp-lcs 0.1.0
Charles Hubain <github@haxelion.eu>
Find the longuest common substrings between the first and second file

USAGE:
    bcmp lcs [FLAGS] [OPTIONS] <first_file> <second_file>

FLAGS:
    -h, --help       Prints help information
    -x               Display a hexdump of every substring
    -V, --version    Prints version information

OPTIONS:
    -a, --algo <ALGO_SPEC>    Sets the algorithm specification. A first letter indicates the algorithm: either h for HashMatch or t for TreeMatch. A number specifying the
                              minimum match length follows it. Example: t1, h4, t7.
    -n, --number <NUMBER>     The number of longuest common substrings to display

ARGS:
    <first_file>     The first file to read data from
    <second_file>    The second file to read data from
```

Unique Substrings
-----------------

```
bcmp-us 0.1.0
Charles Hubain <github@haxelion.eu>
Find the unique substrings of the second file not present in the first

USAGE:
    bcmp us [FLAGS] [OPTIONS] <first_file> <second_file>

FLAGS:
    -h, --help       Prints help information
    -x               Display a hexdump of every substring
    -V, --version    Prints version information

OPTIONS:
    -a, --algo <ALGO_SPEC>    Sets the algorithm specification. A first letter indicates the algorithm: either h for HashMatch or t for TreeMatch. A number specifying the
                              minimum match length follows it. Example: t1, h4, t7.
    -n, --number <NUMBER>     The number of unique substrings to display

ARGS:
    <first_file>     The first file to read data from
    <second_file>    The second file to read data from
```

Patch Set
---------

```
bcmp-ps 0.1.0
Charles Hubain <github@haxelion.eu>
Determine the patch set to build the second file from the first

USAGE:
    bcmp ps [FLAGS] [OPTIONS] <first_file> <second_file>

FLAGS:
    -h, --help       Prints help information
    -x               Display a hexdump of every substring
    -V, --version    Prints version information

OPTIONS:
    -a, --algo <ALGO_SPEC>    Sets the algorithm specification. A first letter indicates the algorithm: either h for HashMatch or t for TreeMatch. A number specifying the
                              minimum match length follows it. Example: t1, h4, t7.

ARGS:
    <first_file>     The first file to read data from
    <second_file>    The second file to read data from
```
