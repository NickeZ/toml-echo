# toml-echo

This is a very simple tool for printing single values out of toml files.

It can be used when packaging rust applications to get the version from the manifest file.

If a tomfile doesn't contain any path segment separators (e.g. "Cargo.toml"), then the file will be
searched after upwards from the current working dir. Otherwise, if the tomlfile argument has
separators (e.g. "./Cargo.toml") it will only be searched for in the current working directory.


## Example usage


```sh
#!/usr/bin/env bash

VERSION=$(toml-echo Cargo.toml package.version)
```
