#![macro_use] extern crate lazy_static;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use stdweb::web::Element;
use stdweb::traits::*;

pub mod error;
pub mod pane_handle;
mod pane;
mod state;
mod storage;
mod style;

pub use error::*;
pub use pane_handle::*;
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
pub fn init_ex(id: Option<&str>, (x,y): (u32,u32), size: Option<(u32, u32)>) -> Result<(), PanesError>  {
    let mut state = get_mut()?;
    if state.is_some() {
        return Err(PanesError::AlreadyInitialized);
    }

    let root = get_root(id)?;
    *state = Some(GlobalState {
        root,
        nodes: PaneHashMap::default(),
        x, y,
        size,
    });
    add_panes_styles_to_document()
}

/// Redefines the global origin for all panes 
/// 
/// The position of all existing panes is changed immediately, regardless of active / inactive status.
/// if panes::init() has been used instead of panes::init_ex(...), the default origin is (0,0).
pub fn reposition(x: u32, y: u32) -> Result<(), PanesError> {
    let mut state = get_mut()?;
    let state = state.as_mut().ok_or(PanesError::NotInitialized)?;
    state.reposition_panes(x,y)
}

/// Redefines the size of the global frame where all panes are within.
/// The panes will change the size AND position proportionally. 
/// 
/// All pane sizes are resized immediately, regardless of active / inactive status.
/// Only has an effect if the size has been defined earlier.
pub fn resize(w: u32, h: u32) -> Result<(), PanesError> {
    let mut state = get_mut()?;
    let state = state.as_mut().ok_or(PanesError::NotInitialized)?;
    state.resize_panes(w,h)
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