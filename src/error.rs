use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// Generic error type for all library calls which may fail.
///
/// Having a common type for all errors can vastly simplify all kinds of call-chains.
pub enum DivError {
    UseAfterDelete,
    NotAllocated,
    NotInitialized,
    AlreadyInitialized,
    Locked,
    MissingBody,
    MissingHead,
    MissingWindow,
    MissingDocument,
    MissingRoot(String),
    MissingChild,
    BrowserError(Box<dyn Error>),
    JsError(String),
    JsCastError,
    UndefinedSize,
}

impl fmt::Display for DivError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DivError::UseAfterDelete =>
                write!(f, "This pane has already been deleted."),
            DivError::NotAllocated =>
                write!(f, "Invalid pane handle, pane has never been allocated."),
            DivError::Locked =>
                write!(f, "The Div-internal data structure has dead-locked itself. This is most likely a bug in Div."),
            DivError::NotInitialized =>
                write!(f, "Called div functions before initializing it. Call div::init() to initialize to body."),
            DivError::AlreadyInitialized=>
                write!(f, "Div has already been initialized, cannot do it again."),
            DivError::MissingBody =>
                write!(f, "No HTML body found."),
            DivError::MissingHead =>
                write!(f, "No HTML head found."),
            DivError::MissingWindow =>
                write!(f, "No Window."),
            DivError::MissingDocument =>
                write!(f, "No Document."),
            DivError::MissingRoot(id) =>
                write!(f, "HTML root element with id = {} not found.", id),
            DivError::MissingChild =>
                write!(f, "DOM child is missing which has been inserted before."),
            DivError::UndefinedSize =>
                write!(f, "Pane has no size."),
            DivError::BrowserError(e) =>
                write!(f, "A browser-call returned an error: {}", e),
            DivError::JsError(msg) =>
                write!(f, "JS error: {}", msg),
            DivError::JsCastError =>
                write!(f, "JS Cast Error"),
        }
    }
}

impl Error for DivError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DivError::BrowserError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<wasm_bindgen::JsValue> for DivError {
    fn from(err: wasm_bindgen::JsValue) -> Self {
        web_sys::console::error_1(&err);
        DivError::JsError(
            "Something in the browser went wrong, check the console error output for more info"
                .to_owned(),
        )
    }
}
