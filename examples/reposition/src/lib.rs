use stdweb::js;
use stdweb::traits::*;
use stdweb::web::event::KeyDownEvent;
use wasm_bindgen::prelude::*;

/**
 * This example show how
 *  div::reposition
 *  div::resize
 *  Pane::reposition
 *  Pane::resize
 * work.
 *
 * To see it in action use the arrow keys, WASD, +/-, 1/2
 *
 * A typical use case for the global feature is resizing or moving the entire
 * area which displays content, for example, entering full screen mode.
 *
 * The HTML inside the div remains untouched (no resizing) but because its container is
 * repositioned and/or resized, it can also change the arrangement of the internal HTML elements.
 */

#[wasm_bindgen(start)]
pub fn main() {

    // Start at position (0,0) with size (350,200)
    let mut x = 0;
    let mut y = 0;
    let w = 350;
    let h = 200;
    div::init_ex(Some("div-root"), (x, y), Some((w, h))).expect("Init failed");

    // Create a pane which shows the total pane area
    let html0 = r#"
    <div style="border:solid; width: 100%; height: 100%; box-sizing: border-box; border: 5px solid black;"></div>
    "#;
    div::new(0, 0, w, h, html0).unwrap();

    // Create two div within to show internal scaling behavior
    let html1 = r#"
    <div style="background-color:red; color: white; font-size: 80px; text-align: center; width: 100%; height: 100%;">
        A
    </div>
    "#;
    let html2 = r#"
    <div style="background-color:blue; color:white; font-size: 80px; text-align: center; width: 100%; height: 100%;">
        B
    </div>
    "#;
    // pane A will have a dynamic position and size
    let (mut ax, mut ay, aw, ah) = (50, 50, 100, 100);
    let pane_a = div::new(ax, ay, aw, ah, html1).unwrap();
    let _pane_b = div::new(200, 50, 100, 100, html2).unwrap();

    // Define control variables for zoom of global area and pane A
    let mut f = 1.0;
    let mut af = 1.0;

    // We are using webstd here to make things easy.
    // Listen to arrow key to move and reposition all div
    stdweb::web::document().add_event_listener(move |e: KeyDownEvent| {
        match e.key().as_str() {
            "ArrowUp" => y = y.saturating_sub(10),
            "ArrowDown" => y += 10,
            "ArrowLeft" => x = x.saturating_sub(10),
            "ArrowRight" => x += 10,
            "+" => f *= 1.5,
            "-" => f /= 1.5,

            "w" => ay = ay.saturating_sub(10),
            "a" => ax = ax.saturating_sub(10),
            "s" => ay += 10,
            "d" => ax += 10,
            "1" => af *= 1.5,
            "2" => af /= 1.5,

            key => {
                js! { @(no_return) console.log("pressed " + @{key}); };
                return;
            }
        }
        div::reposition(x, y).unwrap();
        let w = f * w as f32;
        let h = f * h as f32;
        div::resize(w as u32, h as u32).unwrap();

        let aw = af * aw as f32;
        let ah = af * ah as f32;
        pane_a
            .reposition_and_resize(ax, ay, aw as u32, ah as u32)
            .unwrap();
        // Same as
        // pane_a.reposition(ax,ay).unwrap();
        // pane_a.resize(aw as u32, ah as u32).unwrap();
        // but avoids extra redraw of div
    });
}
