//! Defines operations allowed on DivHandles
//!
//! Almost the entire library interface is defined in this module.

use crate::state;
use crate::*;
use web_sys::{HtmlElement, Node};

/// A light-weight key to refer to the state necessary to manipulate a div.
///
/// This is a unique identifier that will become invalid once the div has been deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DivHandle(pub(crate) usize);

impl DivHandle {
    /// Hides the div and all child nodes.
    ///
    /// The div node is removed from the DOM but it is kept in memory.
    /// Call `delete` to give up memory or call `show` later to display pane again.
    pub fn hide(&self) -> Result<(), DivError> {
        state::exec_mut(|state| state.hide_pane(&self))
    }
    /// Displays a div again after it has been hidden by calling `hide`
    pub fn show(&self) -> Result<(), DivError> {
        state::exec_mut(|state| state.show_pane(&self))
    }
    /// Adjust the relative position of the div.
    ///
    /// The provided parameters are taken in the original scale when initializing,
    /// taking any calls to the global div::resize() into consideration.
    pub fn reposition(&self, x: u32, y: u32) -> Result<(), DivError> {
        state::exec_mut(|state| state.update_pane(&self, Some(x), Some(y), None, None))
    }
    /// Adjust the size of the div.
    ///
    /// The provided parameters are taken in the original scale when initializing,
    /// taking any calls to the global div::resize() into consideration.
    pub fn resize(&self, w: u32, h: u32) -> Result<(), DivError> {
        state::exec_mut(|state| state.update_pane(&self, None, None, Some(w), Some(h)))
    }
    /// Adjust the position and size of the div in a single call, which is slightly more efficient than calling
    /// resize and reposition separately.
    ///
    /// The provided parameters are taken in the original scale when initializing,
    /// taking any calls to the global div::resize() into consideration.
    pub fn reposition_and_resize(&self, x: u32, y: u32, w: u32, h: u32) -> Result<(), DivError> {
        state::exec_mut(|state| state.update_pane(&self, Some(x), Some(y), Some(w), Some(h)))
    }
    /// Set CSS property of div
    pub fn set_css(&self, property: &str, value: &str) -> Result<(), DivError> {
        state::exec(|state| state.nodes.get(self)?.set_css(property, value))
    }
    /// Add a CSS class to the div
    pub fn add_class(&self, css_class: &str) -> Result<(), DivError> {
        state::exec(|state| state.nodes.get(self)?.add_class(css_class))
    }
    /// Remove a CSS class to the div
    pub fn remove_class(&self, css_class: &str) -> Result<(), DivError> {
        state::exec(|state| state.nodes.get(self)?.remove_class(css_class))
    }
    /// Removes a div from the DOM and deletes it
    pub fn delete(&mut self) -> Result<(), DivError> {
        state::exec_mut(|state| state.delete_pane(self))
    }
    /// Get a reference to the DOM element associated with the div.
    /// The provided HTML when creating a new div will be the child node(s) of the returned element.
    pub fn parent_element(&self) -> Result<HtmlElement, DivError> {
        state::exec(|state| state.get_node(&self).map(Clone::clone))
    }
    /// Get a reference to the DOM node created by the provided HTML when creating the pane.
    /// If multiple nodes have been created, the first node is returned.
    pub fn first_inner_node(&self) -> Result<Node, DivError> {
        self.parent_element()?
            .first_child()
            .ok_or(DivError::MissingChild)
    }
}
