use stdweb::web::event::KeyDownEvent;
use stdweb::traits::*;
use stdweb::js;

/**
 * This example show how panes::reposition and panes::resize work.
 * A typical use case for this feature is resizing or moving the area which displays content,
 * for example, entering full screen mode. 
 * 
 * The HTML inside the panes remains untouched (no resizing) but because its container is
 * repositioned and/or resized, it can also change the arrangement of the internal HTML elements.
 */

fn main() {
    stdweb::initialize();

    // Start at position (0,0) with size (350,200)
    let mut x = 0;
    let mut y = 0;
    let w = 350;
    let h = 200;
    panes::init_ex(None, (x,y), Some((w,h))).expect("Init failed");

    // Create a pane which shows the total pane area
    let html0 = r#"
    <div style="border:solid; width: 100%; height: 100%; box-sizing: border-box; border: 5px solid black;"></div>
    "#;
    panes::new_pane(0,0,w,h,html0).unwrap();
    
    // Create two panes within to show internal scaling behavior
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
    panes::new_pane(50,50,100,100,html1).unwrap();
    panes::new_pane(200,50,100,100,html2).unwrap();


    let mut f = 1.0;

    // Listen to arrow key to move and reposition all panes
    stdweb::web::document().add_event_listener(
        move |e: KeyDownEvent|
        {
            match e.key().as_str() {
                "ArrowUp" => y = y.saturating_sub(10),
                "ArrowDown" => y += 10,
                "ArrowLeft" => x = x.saturating_sub(10),
                "ArrowRight" => x += 10,
                "+" => f *= 1.5,
                "-" => f /= 1.5,
                key => { 
                    js!{ @(no_return) console.log("pressed " + @{key}); }; 
                    return;
                }
            }
            panes::reposition(x,y).unwrap();
            let w = f * w as f32;
            let h = f * h as f32;
            panes::resize(w as u32, h as u32).unwrap();
        }
    );
}