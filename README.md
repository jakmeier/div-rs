# Div

*Ad hoc HTML for Rust.*

This crate aims to provide a Rust API to easily create HTML div elements and manage their positions by pixel coordinates. This can be useful in combination with a frame-by-frame rendered canvas. The content of the div should be handled with different libraries.

The code base of the crate is rather small and is built on top of [web-sys](https://crates.io/crates/web-sys)).
The features can be summarized as:
* Create divs as child of an existing DOM node
* Easy management of divs using a `DivHandle` which implements `Copy`
* (Re-)position the divs using absolute coordinates
* Show and hide divs
* Handle resizing of the display by refitting all created divs (e.g. window is resized on desktop, or phone orientation changes)

## Intended Use Cases
* Put HTML over a canvas element that render a game or other graphically intense applications
* Load and manage externally defined components dynamically from within Rust and attach them to the DOM. (Components can be defined with JS or Rust frontend libraries but this features is not very refined, yet.)
* Be a low-level building block for higher-level crates (See the game engine [paddle](https://github.com/jakmeier/Paddle))

## What Div it NOT
* **A GUI library**: Div has no GUI components, but one could potentially build GUI components using Div.
* **A Virtual DOM implementation**: Div only manages a collection of single DOM nodes and has no tracking of their relation to each other, nor does it understand what is inside the node.

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

## Origin and Motivation
My motivation to create Div was to leverage HTML + CSS when using Rust to create games for a browser.
Prior to this, the only way I knew how to do a GUI easily (in Rust running on the browser) was to render everything through general-purpose GUI crates with a WebGL backend. This seemed a bit wasteful to me, as the browser already has excellent built-in support for GUIs.

Instead of rendering fonts in WebGL and creating complex UI libraries in Rust, I wanted to use what the browser supports natively.
This has two main advantages, in my opinion.
1) Smaller size of the shipped Webassembly binary
2) No need to learn the API of a new GUI library, just use HTML + CSS

## License
[MIT / Apache-2.0](LICENSE.md)