# Clapping CLI

## Overview

This project is just experimentations around `clap` and `clap_complete` crates usage.

For the moment, it shows:
- how to generate Bash autocompletion script from a cargo build
- how to separate code for different subcommand into a dedicated cli module

## Tests

```
clapping on master [?] is 📦 v0.1.0 via 🦀 v1.60.0 via ❄️  impure (nix-shell) took 1m51s
✦ ➜ cargo build
   Compiling clapping v0.1.0 (/home/bib0x/git/test/clapping)
warning: completion file is generated: "./completions/clapping.bash"
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s

clapping on master [?] is 📦 v0.1.0 via 🦀 v1.60.0 via ❄️  impure (nix-shell) took 15s
✦ ➜ source ./completions/clapping.bash

clapping on master [?] is 📦 v0.1.0 via 🦀 v1.60.0 via ❄️  impure (nix-shell)
✦ ➜ ./target/debug/clapping<TAB><TAB>
boat    -h      --help  help    room

```
