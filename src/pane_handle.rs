//! Defines operations allowed on PaneHandles
//!
//! Almost the entire library interface is defined in this module.

use crate::state;
use crate::*;
use web_sys::{HtmlElement, Node};

/// External representation of a Pane.
///
/// This is a unique identifier that will become invalid once the pane has been deleted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneHandle(pub(crate) usize);

impl PaneHandle {
    /// Hides the pane and all child nodes.
    ///
    /// The pane root node is removed from the DOM but it is kept in memory.
    /// Call `delete` to give up memory or call `show` later to display pane again.
    pub fn hide(&self) -> Result<(), DivError> {
        state::exec_mut(|state| state.hide_pane(&self))
    }
    /// Displays pane again after it has been hidden by calling `hide`
    pub fn show(&self) -> Result<(), DivError> {
        state::exec_mut(|state| state.show_pane(&self))
    }
    /// Adjust the relative position of the pane.
    ///
    /// The provided parameters are taken in the original scale when initializing,
    /// taking any calls to the global div::resize() into consideration.
    pub fn reposition(&self, x: u32, y: u32) -> Result<(), DivError> {
        state::exec_mut(|state| state.update_pane(&self, Some(x), Some(y), None, None))
    }
    /// Adjust the size of the pane.
    ///
    /// The provided parameters are taken in the original scale when initializing,
    /// taking any calls to the global div::resize() into consideration.
    pub fn resize(&self, w: u32, h: u32) -> Result<(), DivError> {
        state::exec_mut(|state| state.update_pane(&self, None, None, Some(w), Some(h)))
    }
    /// Adjust the position and size of the pane in a single call, which is slightly more efficient than calling
    /// resize and reposition separately.
    ///
    /// The provided parameters are taken in the original scale when initializing,
    /// taking any calls to the global div::resize() into consideration.
    pub fn reposition_and_resize(&self, x: u32, y: u32, w: u32, h: u32) -> Result<(), DivError> {
        state::exec_mut(|state| state.update_pane(&self, Some(x), Some(y), Some(w), Some(h)))
    }
    /// Removes a pane from the DOM and deletes it
    pub fn delete(&mut self) -> Result<(), DivError> {
        state::exec_mut(|state| state.delete_pane(self))
    }
    /// Get a reference to the DOM element associated with the pane.
    /// The provided HTML when creating a new pane will be the child node(s) of the returned element.
    pub fn parent_element(&self) -> Result<HtmlElement, DivError> {
        state::exec(|state| state.get_node(&self).map(Clone::clone))
    }
    /// Get a reference to the DOM node created by the provided HTML when creating the pane.
    /// If multiple nodes have been created, the first node is returned.
    ///
    /// The returned [`Node`] is from the crate [`stdweb`], allowing library users to
    /// escape the Div crate and access the DOM directly.
    ///
    /// TODO: Example
    ///
    /// [`Node`]: https://docs.rs/stdweb/*/stdweb/web/struct.Node.html
    /// [`stdweb`]: https://docs.rs/stdweb/*/stdweb/
    pub fn first_inner_node(&self) -> Result<Node, DivError> {
        self.parent_element()?
            .first_child()
            .ok_or(DivError::MissingChild)
    }
}
