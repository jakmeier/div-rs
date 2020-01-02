use super::*;

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