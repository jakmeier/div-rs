use crate::{error::*, utils::doc};

pub(crate) fn add_panes_styles_to_document() -> Result<(), PanesError> {
    let css = r#"
    .pane {
        position: absolute;
        display: unset;
        z-index: 1;
        overflow: hidden;
    }
    "#;

    let head = doc()?.head().ok_or(PanesError::MissingHead)?;
    let style = doc()?.create_element("style")?;
    style.set_attribute("type", "text/css")?;
    style.set_text_content(Some(css));
    head.append_child(&style)?;
    Ok(())
}
