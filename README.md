## About

This is an example [Zellij][zellij] plugin in Rust.
It's a very simple event logger that can detect Zellij mode changes, display them to the screen, and eventually write them to a file.

You can learn more about developing plugins in the [Zellij Documentation][docs], which walks through writing this example plugin.

[zellij]: https://github.com/zellij-org/zellij
[docs]: https://zellij.dev/documentation/plugins.html

## Usage

### Build with `cargo` and Test in Zellij

```sh
# If you don't have Zellij installed already
cargo install zellij
# Building the plugin
cargo build
# Running in Zellij
zellij -l plugin
```
