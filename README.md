# floating-notes

Markdown note-taking app. Notes are plain `.md` files in a folder you choose.

Two screens: a small popup to add/edit notes, and a fullscreen view where notes float across the screen.

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
