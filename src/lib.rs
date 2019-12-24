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
    let mut state = get_mut()?;
    if state.is_some() {
        return Err(PanesError::AlreadyInitialized);
    }
    *state = Some(GlobalState::mount_body()?);
    add_panes_styles_to_document()
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
pub fn init_ex(id: &str, x: u32, y: u32, width: u32, height: u32) -> Result<(), PanesError>  {
    let mut state = get_mut()?;
    if state.is_some() {
        return Err(PanesError::AlreadyInitialized);
    }

    let document = stdweb::web::document();
    let root = document.get_element_by_id(id).ok_or(PanesError::MissingRoot(id.to_owned()))?;
    *state = Some(GlobalState {
        root,
        nodes: PaneHashMap::default(),
        x, y,
        width: Some(width), height: Some(height),
    });
    add_panes_styles_to_document()
}

impl<PS> GlobalState<PS> 
where PS: PaneStorage + Default
{
    fn mount_body() -> Result<Self, PanesError> {
        let document = stdweb::web::document();
        let root = document.body().ok_or(PanesError::MissingBody)?.into();
        Ok(GlobalState {
            root,
            nodes: Default::default(),
            x: 0, y: 0,
            width: None, height: None
        })
    }
}