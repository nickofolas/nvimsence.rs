# `nvimsence.rs`

A NeoVim plugin for generating Discord Rich Presences, and written in Rust.

### Installation
**Vundle**
```vim
Plugin 'arcaaaa/nvimsence.rs'
```

This installs the vimscript requirements. The bulk of the plugin is within the Rust binary.

Precompiled binaries are available in the releases of this repository. If your system matches one of the binaries, download it and move it into the plugin's directory (ex `~/.vim/bundle/nvimsence.rs/`)

Finally, in your `vimrc`, add:
```vim
let g:rich_presence_binary = " the name of the binary that you downloaded
```

If your system if not included in the pre-compiled binaries, first ensure you have the Rust toolchain installed on your system. Then, change directories into the plugin's directory (see above). Run `cargo build --release`, and add:
```vim
let g:rich_presence_binary = "target/release/nvimsence-rs"
```
to your `vimrc`. At this point, the plugin should work.

### Configuration
`nvimsence.rs` supports some basic configuration, shown below (with default values):
```vim
" Available variables:
" * {project} - The name of the active project. This will either be the parent directory, or, if the file is part of a Git tree, the name of the Git repository
" * {filename} - The name of the active file. Includes the file extension
" * {filesize} - The size of the file, given in MiB/GiB etc
" * {lines} - The total number of lines in the active file
" * {none} - Do not show state in the rich presence
let g:nvimsence_details = "{project}/{filename}"

" Available variables:
" * {project} - The name of the active project. This will either be the parent directory, or, if the file is part of a Git tree, the name of the Git repository
" * {filename} - The name of the active file. Includes the file extension
" * {filesize} - The size of the file, given in MiB/GiB etc
" * {lines} - The total number of lines in the active file
" * {none} - Do not show state in the rich presence
let g:nvimsence_state = "{filesize} [{lines} LOC]"

" Boolean [0|1]
let g:nvimsence_show_elapsed = 1

" Boolean [0|1]
" NOTE: In order for buttons to be displayed, the Fugitive plugin must be installed so that the remote URL an be fetched
" NOTE: Fetches the URL for the `origin` remote
let g:nvimsence_show_buttons = 1
```

Notes:
* `nvimsence.rs` is currently a WIP project
