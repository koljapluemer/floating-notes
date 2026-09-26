set shell := ["bash", "-euo", "pipefail", "-c"]

pkg := "floating-notes"
bundle_dir := "src-tauri/target/release/bundle"

default:
    @just --list

# Run the app with hot reload
dev: _node_modules
    npm run tauri dev

# Build the native package for this distro (.deb on Ubuntu/Debian, .rpm on Fedora)
build: _check_deps _node_modules
    #!/usr/bin/env bash
    set -euo pipefail
    fmt=$(just _pkg_format)
    rm -rf "{{bundle_dir}}/$fmt"
    npm run tauri build -- --bundles "$fmt"

# Remove any installed copy, build fresh, and install it
reinstall: _check_deps
    #!/usr/bin/env bash
    set -euo pipefail
    npm ci
    just build
    just uninstall
    fmt=$(just _pkg_format)
    file=$(ls "{{bundle_dir}}/$fmt/"*."$fmt")
    echo "Installing $file"
    case "$fmt" in
        deb)
            # apt's sandbox user (_apt) can't read inside $HOME, so stage the file somewhere it can
            tmp=$(mktemp -d)
            trap 'rm -rf "$tmp"' EXIT
            cp "$file" "$tmp/"
            chmod 755 "$tmp"; chmod 644 "$tmp/"*.deb
            sudo apt-get install -y --reinstall "$tmp/$(basename "$file")"
            ;;
        rpm) sudo dnf install -y "./$file" ;;
    esac

# Remove the installed package (no-op if not installed)
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    case "$(just _pkg_format)" in
        deb) if dpkg -s {{pkg}} &>/dev/null; then sudo apt-get remove -y {{pkg}}; fi ;;
        rpm) if rpm -q {{pkg}} &>/dev/null; then sudo dnf remove -y {{pkg}}; fi ;;
    esac

# Install system libraries Tauri needs to build
deps:
    #!/usr/bin/env bash
    set -euo pipefail
    case "$(just _pkg_format)" in
        deb)
            sudo apt-get update
            sudo apt-get install -y build-essential curl wget file pkg-config \
                libwebkit2gtk-4.1-dev libxdo-dev libssl-dev \
                libayatana-appindicator3-dev librsvg2-dev
            ;;
        rpm)
            sudo dnf install -y gcc gcc-c++ make curl wget file pkgconf-pkg-config \
                webkit2gtk4.1-devel openssl-devel libxdo-devel \
                libappindicator-gtk3-devel librsvg2-devel
            ;;
    esac

# Remove build outputs and node_modules
clean:
    rm -rf dist node_modules
    cd src-tauri && cargo clean

_node_modules:
    @[ -d node_modules ] || npm install

_check_deps:
    #!/usr/bin/env bash
    set -euo pipefail
    for tool in cargo npm; do
        command -v "$tool" >/dev/null || { echo "error: $tool not found (install Rust via rustup / Node.js)"; exit 1; }
    done
    pkg-config --exists webkit2gtk-4.1 || just deps

_pkg_format:
    #!/usr/bin/env bash
    . /etc/os-release
    case " $ID ${ID_LIKE:-} " in
        *" debian "*|*" ubuntu "*) echo deb ;;
        *" fedora "*|*" rhel "*) echo rpm ;;
        *) echo "error: unsupported distro '$ID' (need Ubuntu/Debian or Fedora)" >&2; exit 1 ;;
    esac
