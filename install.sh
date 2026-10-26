#!/usr/bin/env bash
# New Mac bootstrap: Homebrew, rustup (Brewfile has cargo entries), packages, symlinks.
set -e
cd "$(dirname "$0")"

command -v brew >/dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
eval "$(/opt/homebrew/bin/brew shellenv)"

[ -f "$HOME/.cargo/env" ] || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
. "$HOME/.cargo/env"

brew bundle --file=Brewfile
./deploy.sh MANIFEST

chmod 700 "$HOME/.ssh" "$HOME/.gnupg"