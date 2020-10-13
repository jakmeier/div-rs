use div::PaneHandle;
use stdweb::web::set_timeout;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    div::init_to("div-root").expect("Init failed");

    // Create two new div with some HTML in it
    let html0 = r#"
    <div style="background-color:grey; color: white; height: 100%;">
        <div style="text-align: end; position: absolute; bottom: 0; right: 0;">
            Hi!
        </div>
    </div>
    "#;
    let html1 = r#"
    <div style="background-color:grey; color:white; height: 100%;">
        <div>
            Bye!
        </div>
    </div>
    "#;
    let pane0 = div::new_pane(100, 100, 100, 100, html0).unwrap();
    let pane1 = div::new_pane(200, 200, 100, 100, html1).unwrap();

    toggle(pane0, pane1);
}

// Function that takes to div, shows the first and hides the second
// and then calls itself again delayed, with the two div swapped
fn toggle(a: PaneHandle, b: PaneHandle) {
    a.show().expect("Error");
    b.hide().expect("Error");
    let closure = move || {
        toggle(b, a);
    };
    set_timeout(closure, 1000);
}
