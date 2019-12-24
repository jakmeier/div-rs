use stdweb::web::set_timeout;
use panes::PaneHandle;

fn main() {
    stdweb::initialize();
    panes::init().expect("Init failed");

    // Create two new panes with some HTML in it
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
    let pane0 = panes::new_pane(100,100,100,100,html0).unwrap();
    let pane1 = panes::new_pane(200,200,100,100,html1).unwrap();

    toggle(pane0, pane1);
}

// Function that takes to panes, shows the first and hides the second 
// and then calls itself again delayed, with the two panes swapped
fn toggle(a: PaneHandle, b: PaneHandle) {
    a.show().expect("Error");
    b.hide().expect("Error");
    let closure = move || { toggle(b,a); };
    set_timeout(closure, 1000);
}