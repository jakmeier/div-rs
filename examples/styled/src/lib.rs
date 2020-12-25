/**
 * An example for using CSS styles inside Rust
 *
 * Usually we would have your styles served from a .css or inside a svelte component.
 * But if you want to, you can also apply CSS to a pane from within Rust.
 */
use stdweb::web::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    div::init_to("div-root").expect("Init failed");

    // Defining classes
    let global_css = r#"
    .white-on-black {
        background-color: black;
        color: white;
        padding: 30px;
    }
    .annoying-orange {
        background-color: orange;
    }
    "#;
    // Add class definitions to <head>
    add_document_styles(global_css);

    // Now to the part that uses DIV-RS to style panes.

    // Pane 0 using CSS classes
    let html0 = "Pane 0<br>Styled with classes";
    let classes = ["white-on-black"];
    let css: [(&str, &str); 0] = [];
    let _pane0 = div::new_styled(100, 100, 200, 100, html0, &classes, &css).unwrap();

    // Pane 1 with inline styles on pane
    let html1 = "Pane 1<br>Stlyed with classes and inline CSS";
    let classes = ["annoying-orange"];
    let css = [
        ("font-weight", "bold"),
        ("text-decoration", "underline"),
        ("padding", "5px"),
    ];
    let _pane1 = div::new_styled(125, 300, 200, 100, html1, &classes, &css).unwrap();
}

// Small helper function, only for the example. Uses stdweb, DIV-RS does not really help you with this part of CSS.
// It is not recommended to add classes like this but it is useful here to keep everything in a single file.
fn add_document_styles(css: &str) {
    let head = document().head().unwrap();
    let style = document().create_element("style").unwrap();
    style.set_attribute("type", "text/css").unwrap();
    style.append_html(css).unwrap();
    head.append_child(&style);
}
