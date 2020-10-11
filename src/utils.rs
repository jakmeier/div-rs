use crate::PanesError;

pub(crate) fn doc() -> Result<web_sys::Document, PanesError> {
    let window = web_sys::window().ok_or(PanesError::MissingWindow)?;
    window.document().ok_or(PanesError::MissingDocument)
}
