# `nvimsence.rs`

A NeoVim plugin for generating Discord Rich Presences, and written in Rust.

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
* Installation currently requires compilation after downloading. A future goal is to distribute pre-compiled binaries to simplify the installation process
