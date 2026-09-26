# floating-notes

Floating note viewer. Notes are JSON files in a folder you choose.

The main screen floats notes across the window. Settings controls the notes folder and animation timing.

## Dev

```sh
npm install
npm run tauri dev
```

## Build

```sh
npm run tauri build
```

Output: `src-tauri/target/release/bundle/`

## Install (Linux)

```sh
sudo dpkg -i src-tauri/target/release/bundle/deb/floating-notes_*.deb
```
