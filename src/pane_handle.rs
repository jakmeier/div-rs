//! Defines operations allowed on PaneHandles
//! 
//! Almost the entire library interface is defined in this module.

use crate::*;
use stdweb::web::HtmlElement;
use stdweb::unstable::TryInto;

/// External representation of a Pane.
/// 
/// This is a unique identifier that will become invalid once the pane has been deleted.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct PaneHandle(pub (crate) usize);

/// Creates a new pane at the defined position with the given HTML as content.
/// Use the returned PaneHandle to manipulate the pane.
pub fn new_pane(x: u32, y: u32, w: u32, h: u32, html: &str) -> Result<PaneHandle, PanesError> {
    let mut state = get_mut()?;
    let ph = state.as_mut().ok_or(PanesError::NotInitialized)?.new_pane(x,y,w,h,html)?;
    Ok(ph)
}

impl PaneHandle {
    /// Hides the pane and all child nodes.
    /// 
    /// The pane root node is removed from the DOM but it is kept in memory.
    /// Call `delete` to give up memory or call `show` later to display pane again.
    pub fn hide(&self) -> Result<(), PanesError> {
        get_mut()?.as_mut().ok_or(PanesError::NotInitialized)?.hide_pane(&self)?;
        Ok(())
    }
    /// Displays pane again after it has been hidden by calling `hide`
    pub fn show(&self) -> Result<(), PanesError> {
        get_mut()?.as_mut().ok_or(PanesError::NotInitialized)?.show_pane(&self)?;
        Ok(())
    }
    /// Removes a pane from the DOM and deletes it
    pub fn delete(self) -> Result<(), PanesError> {
        let _pane = get_mut()?.as_mut().ok_or(PanesError::NotInitialized)?.delete_pane(self);
        Ok(())
    }
    /// Get a reference to a DOM node associated with the pane.
    /// 
    /// The returned [`HtmlElement`] is from the crate [`stdweb`], allowing library users to
    /// escape the Panes crate and access the DOM directly.
    /// 
    /// The provided HTML when creating a new pane will be the child node(s) of the returned element.
    /// 
    /// TODO: Example
    /// 
    /// [`HtmlElement`]: https://docs.rs/stdweb/*/stdweb/web/struct.HtmlElement.html
    /// [`stdweb`]: https://docs.rs/stdweb/*/stdweb/
    pub fn get_node(&self) -> Result<HtmlElement, PanesError> {
        get()?.as_ref()
            .ok_or(PanesError::NotInitialized)?
            .get_node(&self)?.clone()
            .try_into()
            .map_err(|e|PanesError::BrowserError(Box::new(e)))
    }
}