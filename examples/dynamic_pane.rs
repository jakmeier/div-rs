use stdweb::js;
use stdweb::web::*;
use stdweb::unstable::TryInto;
fn main() {
    stdweb::initialize();
    panes::init().unwrap();

    // Create a new pane with a div as grid container.
    // Because we want low-level access afterwards, it is important here that 
    // the div starts at the first line. Otherwise, there will be a text node before the div
    let html = 
    r#"<div style="display: grid; grid-template-columns: repeat(5, 1fr); grid-gap: 10px; overflow: auto;"></div>"#;
    let pane = panes::new_pane(100,100,550,550,html).unwrap();

    // Get low-level access to the DOM node representing the grid container
    let node = pane.get_first_inner_node().unwrap();


    /* Now we are in webstd territory */

    // UpCast to an HtmlElement
    let table: HtmlElement = node.try_into().expect("Should be HTML");

    // Append a single element to the grid
    table.append_html(
        r#"<div style="background-color: blue; height: 100px;">"#
    ).unwrap();

    // Clone table reference so we can move it into closure
    let cloned_table = table.clone();
    // Append more dynamically, using a closure registered as event listener on space bar key
    let append_closure = move || {
        cloned_table.append_html(
            r#"<div style="background-color: blue; height: 100px;">"#
        ).unwrap();
    };
    // Registration is here done using the webstd js macro
    js!{
        document.addEventListener("keydown", event => {
            if (event.code === "Space") {
              @{append_closure}()
            }
          })
    }

    // Also register to remove one on backspace
    let delete_closure = move || {
        if let Some(child) = table.last_child() {
            table.remove_child(&child).unwrap();
        }
    };
    // Registration is here done using the webstd js macro
    js!{
        document.addEventListener("keydown", event => {
            if (event.code === "Backspace") {
              @{delete_closure}()
            }
          })
    }
}