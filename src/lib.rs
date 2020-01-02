#![macro_use] extern crate lazy_static;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use stdweb::web::Element;
use stdweb::traits::*;

pub mod error;
pub mod pane_handle;
pub mod global;
mod pane;
mod state;
mod storage;
mod style;

pub use error::*;
pub use pane_handle::*;
pub use global::*;
use storage::{PaneStorage, PaneHashMap};
use state::*;
use style::*;

/// Mounts the panes to the HTML body
pub fn init() -> Result<(), PanesError> {
    init_ex(None,(0,0),None)
}

/// Extended initialization function.
/// Mounts the panes as a child of the HTML element with the defined ID.
/// The specified dimensions restrict the area in which panes are visible.
/// # Example
/// ```
/// let width = 1280
/// let height = 720
/// panes::init_ex("panes-root", 0, 0, width, height);
/// ```
pub fn init_ex(id: Option<&str>, pos: (u32,u32), size: Option<(u32, u32)>) -> Result<(), PanesError>  {
    let mut state = get_mut()?;
    if state.is_some() {
        return Err(PanesError::AlreadyInitialized);
    }

    let root = get_root(id)?;
    *state = Some(GlobalState {
        root,
        nodes: PaneHashMap::default(),
        pos,
        size,
        zoom: (1.0, 1.0),
    });
    add_panes_styles_to_document()
}

fn get_root(id: Option<&str>) -> Result<Element, PanesError> {
    let document = stdweb::web::document();
    let element = 
    if id.is_some() {
        document.get_element_by_id(id.unwrap()).ok_or(PanesError::MissingRoot(id.unwrap().to_owned()))?
    } else {
        document.body().ok_or(PanesError::MissingBody)?.into()
    };
    Ok(element)
}

/// Creates a new pane at the defined position with the given HTML as content.
/// Use the returned PaneHandle to manipulate the pane.
pub fn new_pane(x: u32, y: u32, w: u32, h: u32, html: &str) -> Result<PaneHandle, PanesError> {
    let mut state = get_mut()?;
    let ph = state.as_mut().ok_or(PanesError::NotInitialized)?.new_pane(x,y,w,h,html)?;
    Ok(ph)
}