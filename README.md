# floating-notes

Floating note viewer. Notes are JSON files in a folder you choose.

The main screen floats notes across the window. Settings controls the notes folder and animation timing.

## Dev

Requires Rust, Node.js and [just](https://github.com/casey/just).

```sh
just deps   # system libraries (apt on Ubuntu, dnf on Fedora); run automatically if missing
just dev    # run with hot reload
```

## Install (Ubuntu / Fedora)

```sh
just reinstall
```

Builds a `.deb` or `.rpm` for the current distro, removes any installed copy, and installs the new one. `just uninstall` removes it.
