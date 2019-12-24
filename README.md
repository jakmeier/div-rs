# Panes

This crate aims to provide a Rust API to easily place HTML elements by pixel coordinates.
It works with absolute positions and can therefore be used to place HTML text elements over a canvas element.

## Examples

TODO

### Running the examples
To run the examples included in the library, it's recommended to install [cargo-web](https://github.com/koute/cargo-web) and then simply call `cargo web start --example hello-world`. See the [examples](./examples) folder for all available examples. 

## Implementation policy for Panes
* Compatibility with stable Rust
* Light-weight (only major dependency is [stdweb](https://github.com/koute/stdweb))
* Integration to any WASM project should be possible without much hassle

## What Panes it NOT
* A complete GUI library
* A Virtual DOM implementation

## Contribution
Pull-requests are always welcome!
Please just try to adhere to the listed implementation policies or contact me first why you think it is necessary to break some of them.

## License
[MIT / Apache-2.0](LICENSE.md)