# rstree

This little application implements the `tree` command in Rust. There might be a lot of tree rewrites in Rust out there. This project is intended to learn the language and try out as many things as possible.
If you find something to change, issues or anything else feel free to open an issue.

It is also very likely that this implementation is not as idiomatic as it could be. If you have suggestions on how to improve my code I would be happy to hear them !

## Usage

```
rstree 0.1.0
Alexander Weiß
A tree clone written in Rust

USAGE:
    rstree [FLAGS] [OPTIONS] [DIRECTORY]

FLAGS:
    -c               Colorize output
    -h, --help       Prints help information
    -a               Print all files, including hidden
    -V, --version    Prints version information

OPTIONS:
    -l <level>        Set the depth of the iteraton [default: 0]

ARGS:
    <DIRECTORY>    Directory to start with [default: .]
```

## Todo

Implement the following options and flags:

- -d List directories only.
- -l Follow symbolic links like directories.
- -f Print the full path prefix for each file.
- -P pattern List only those files that match the pattern given.
- -I pattern Do not list files that match the given pattern.
- -J Prints out an JSON representation of the tree.

These is a non exhaustive list of the original `tree` options. I might extend the list as I proceed with the implementation.
