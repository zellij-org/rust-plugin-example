## About

This is an example [Zellij][zellij] plugin in Rust. It can be used as a template to start developing your own plugins.

More about Zellij plugins: [Zellij Documentation][docs]

[zellij]: https://github.com/zellij-org/zellij
[docs]: https://zellij.dev/documentation/plugins.html

## Development

*Note*: you will need to have `wasm32-wasi` added to rust as a target to build the plugin. This can be done with `rustup target add wasm32-wasi`.

### With the Provided Layout

Run `zellij -l zellij.kdl` at the root of this repository. This will open a development environment that will help you develop the plugin inside Zellij.

It can also be used if you prefer developing outside of the terminal - in this case you should ignore the `$EDITOR` pane and use your IDE instead.

### Otherwise

1. Build the project: `cargo build`
2. Load it inside a running Zellij session: `zellij action start-or-reload-plugin file:target/wasm32-wasi/debug/rust-plugin-example.wasm`
3. Repeat on changes (perhaps with a `watchexec` or similar command to run on fs changes).
