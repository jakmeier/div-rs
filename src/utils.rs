use crate::DivError;

pub(crate) fn doc() -> Result<web_sys::Document, DivError> {
    let window = web_sys::window().ok_or(DivError::MissingWindow)?;
    window.document().ok_or(DivError::MissingDocument)
}
