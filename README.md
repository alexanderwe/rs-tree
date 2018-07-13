# rstree

This little application implements the `tree` command in Rust. With this project I want to learn the language and find out how to do things.
It is also very likely that this implementation is not as idiomatic as it could be. If you have suggestions on how to improve my code I would be happy to hear them. Feel free to create an issue and let me know what I can improve.

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
    -L <level>        Set the depth of the iteraton [default: 0]

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

## Contribution

Since this project is intended to learn the language I might not accept every pull request because I want to implement things on my own. But as I said if you have suggestions I would appreciate if you leave an issue.

Stay rusty 🦀
