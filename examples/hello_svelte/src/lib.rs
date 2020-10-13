use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    div::init_to("div-root").unwrap();
    set_panic_hook();

    // Create a new pane at offset (100,100) from body
    // with size 500px/500px and then create a single
    // text node inside it with an external class stored in TODO
    const X: u32 = 100;
    const Y: u32 = 100;
    const W: u32 = 500;
    const H: u32 = 500;
    let class = div::JsClass::preregistered("MyComponent")
        .expect("JS class Test has not been registered properly");
    div::from_js_class(X, Y, W, H, class).unwrap();

    /* Alternative that loads classes from a separate JS file instead of registering in the JS code. */
    // let future = async {
    //     let class = div::load_js_class("MyComponent", "./some_file.js").unwrap().await;
    //     div::from_js_class(X, Y, W, H, class).unwrap();
    // };
    // wasm_bindgen_futures::spawn_local(future);
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
