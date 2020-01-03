/**
 * An example for using CSS styles with panes
 */

use stdweb::web::*;

pub fn main() {    
    stdweb::initialize();
    panes::init().expect("Init failed");

    // Usually you would have your styles served from a .css file but for
    // this example it is more compact to add it dynamically
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
    add_document_styles(global_css);


    // Pane 0 using CSS classes
    let html0 = "Pane 0</p>";
    let classes = ["white-on-black"];
    let css: [(&str, &str);0] = [];
    let _pane0 = panes::new_styled_pane(
        100,100,200,100,
        html0,
        &classes,
        &css,
    ).unwrap();
    
    // Pane 1 with inline styles on pane
    let html1 = "Pane 1";
    let classes = ["annoying-orange"];
    let css = [
        ("font-weight","bold"),
        ("text-decoration","underline"),
        ("padding","5px"),
    ];
    let _pane1 = panes::new_styled_pane(
        100,300,200,100,
        html1,
        &classes,
        &css,
    ).unwrap();    

}

fn add_document_styles(css: &str) {
    let head = document().head().unwrap();
    let style = document().create_element("style").unwrap();
    style.set_attribute("type", "text/css").unwrap();
    style.append_html(css).unwrap();
    head.append_child(&style);
}