use crate::{error::*, utils::doc};

pub(crate) fn add_div_styles_to_document() -> Result<(), DivError> {
    let css = r#"
    .pane {
        position: absolute;
        display: unset;
        z-index: 1;
        overflow: hidden;
    }
    "#;

    let head = doc()?.head().ok_or(DivError::MissingHead)?;
    let style = doc()?.create_element("style")?;
    style.set_attribute("type", "text/css")?;
    style.set_text_content(Some(css));
    head.append_child(&style)?;
    Ok(())
}
