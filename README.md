# Div

This crate aims to provide a Rust API to easily create HTML elements, position them by pixel coordinates, and load content 
defined in JavaScript into the element.

## Intended Use Cases
* Put HTML over a canvas element that render e.g. a game
* Load Svelte or vanilla JS components dynamically from within Rust
* Be a low-level building block for higher-level crates

## What Div it NOT
* A complete GUI library
* A Virtual DOM implementation


## Examples

The examples in this crate are hosted online: [div-rs Examples](https://div.paddlers.ch/)

Have a look at the code in example directory. The best way is to clone the repository and run it locally, so you can play around with the code.

You need npm, webpack, and wasm-pack for the examples to run on your machine.
```
git clone https://github.com/jakmeier/div-rs.git
cd div-rs/examples/www;
npm run build;
npm run start;
```

## License
[MIT / Apache-2.0](LICENSE.md)