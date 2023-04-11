# search-edit

A command-line utility that allows the user to select a file using a fuzzy search algorithm and open it in their preferred text editor.

(Yes, I know this should be a bash script. In fact, [it was](https://gist.github.com/Sciencentistguy/0bc58405b9f37dcbc544a5f879d3ff91)! I needed to use it on Windows, and rewriting it in Rust was the easiest way I could think of to make it cross-platform.)

## Dependencies

- A command-line fuzzy finder, by default `fzf`
- `bat` command-line utility

## Usage

`search-edit` can be configured with environment variables or command-line arguments, with the latter taking priority.

See `search-edit --help` for more information

I would reccommend setting the `FZF_DEFAULT_COMMAND` environment variable to a value such as `fd --type f` if you're using `fzf`. (This requires the [fd](https://github.com/sharkdp/fd) utility)

---

Available under the terms of version 2.0 of the Mozilla Public Licence.
