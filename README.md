# ripper

Just another cli tool to manage Linux trash bin written in Rust

# Features/TODO

- [x] Delete file/directory (to trash bin)
- [x] Recover last file/directory
- [x] Recover specific file/directory
- [x] List **all** trashed files/directories

# Installation
Download the binary from releases

## Building:
    git clone https://github.com/itslu666/ripper.git
    cd ripper
    cargo build --release

The executable will be in `target/release`.

# Usage
    ripper [OPTIONS] [files]...

Option | Effect
--|--
-d, --dig | Dig out (List) all trashed files
-r, --revive | Revive (Recover) files from trash bin
-b, --b | Bury (Delete) a file
-h, --help | Print help
-V, --version | Print version

You can also delete files without the `-b, --bury` tag.
You can also recover the latest file without the naming a file after `-r, --revive`.