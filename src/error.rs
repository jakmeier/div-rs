use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// Generic error type for all library calls which may fail.
///
/// Having a common type for all errors can vastly simplify all kinds of call-chains.
pub enum PanesError {
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

impl fmt::Display for PanesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PanesError::UseAfterDelete =>
                write!(f, "This pane has already been deleted."),
            PanesError::NotAllocated =>
                write!(f, "Invalid pane handle, pane has never been allocated."),
            PanesError::Locked =>
                write!(f, "The Panes-internal data structure has dead-locked itself. This is most likely a bug in Panes."),
            PanesError::NotInitialized =>
                write!(f, "Called panes functions before initializing it. Call panes::init() to initialize to body."),
            PanesError::AlreadyInitialized=>
                write!(f, "Panes has already been initialized, cannot do it again."),
            PanesError::MissingBody =>
                write!(f, "No HTML body found."),
            PanesError::MissingHead =>
                write!(f, "No HTML head found."),
            PanesError::MissingWindow =>
                write!(f, "No Window."),
            PanesError::MissingDocument =>
                write!(f, "No Document."),
            PanesError::MissingRoot(id) =>
                write!(f, "HTML root element with id = {} not found.", id),
            PanesError::MissingChild =>
                write!(f, "DOM child is missing which has been inserted before."),
            PanesError::UndefinedSize =>
                write!(f, "Pane has no size."),
            PanesError::BrowserError(e) =>
                write!(f, "A browser-call returned an error: {}", e),
            PanesError::JsError(msg) =>
                write!(f, "JS error: {}", msg),
            PanesError::JsCastError =>
                write!(f, "JS Cast Error"),
        }
    }
}

impl Error for PanesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PanesError::BrowserError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<wasm_bindgen::JsValue> for PanesError {
    fn from(err: wasm_bindgen::JsValue) -> Self {
        web_sys::console::error_1(&err);
        PanesError::JsError(
            "Something in the browser went wrong, check the console error output for more info"
                .to_owned(),
        )
    }
}
