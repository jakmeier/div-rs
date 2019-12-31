use stdweb::web::*;
use crate::error::*;

pub fn add_panes_styles_to_document() -> Result<(), PanesError> {

    let css = r#"
    .pane {
        position: absolute;
        display: unset;
        z-index: 1;
        overflow: hidden
    }
    "#;

    let head = document().head().ok_or(PanesError::MissingHead)?;
    let style = document().create_element("style").map_err(|e|PanesError::BrowserError(Box::new(e)))?;
    style.set_attribute("type", "text/css").map_err(|e|PanesError::BrowserError(Box::new(e)))?;
    style.append_html(css).map_err(|e|PanesError::BrowserError(Box::new(e)))?;
    head.append_child(&style);
    Ok(())
}