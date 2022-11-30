# furiosa-version
A simple CLI tool to print out version strings of libraries.

Installation:
```
cargo install --git https://github.com/furiosa-ai/furiosa-version.git
```

Synopsis:
```
target/release/furiosa-version 
Usage: furiosa-version [OPTIONS] <name>

Arguments:
  <name>  an library to get (available names: libhal, libruntime, libcompiler)

Options:
      --version     Get a version string
      --hash        Get a git hash string
      --build-time  Get a build time
  -h, --help        Print help information

Examples:

  # Print a git hash of libcompiler
  furiosa-version libcompiler --hash

  # Print a version and build time of libhal
  furiosa-version libhal --version --build-time
```

Examples:
```
$ target/release/furiosa-version libcompiler
0.9.0-dev 37d8500e4 2022-11-29T21:58:26Z

$ target/release/furiosa-version libhal --version --build-time
2.0 2022-11-09T01:10:52Z
```