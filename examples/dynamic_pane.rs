use stdweb::js;
use stdweb::web::*;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::KeyDownEvent;

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
    let node = pane.first_inner_node().unwrap();


    /* Now we are in webstd territory */

    // UpCast to an HtmlElement
    let table: HtmlElement = node.try_into().expect("Should be HTML");

    // Append a single element to the grid
    table.append_html(
        r#"<div style="background-color: blue; height: 100px;">"#
    ).unwrap();

    // Registration event listener to add / remove panes dynamically on space / backspace
    document().add_event_listener(
            move |e: KeyDownEvent|
            match e.key().as_str() {
                " " | "Enter" => 
                    table.append_html(
                        r#"<div style="background-color: blue; height: 100px;">"#
                    ).unwrap(),
                "Backspace" =>
                    if let Some(child) = table.last_child() {
                        table.remove_child(&child).unwrap();
                    },
                key => js!{ @(no_return) console.log("pressed " + @{key}); },
            }  
    );
}