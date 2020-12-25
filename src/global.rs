use super::*;

/// Redefines the global origin for all div
///
/// The position of all existing div is changed immediately, regardless of active / inactive status.
/// if div::init() has been used instead of div::init_ex(...), the default origin is (0,0).
pub fn reposition(x: u32, y: u32) -> Result<(), DivError> {
    state::exec_mut(|state| state.global_reposition(x, y))
}

/// Redefines the size of the global frame where all div are within.
/// The div will change the size AND position proportionally.
///
/// All pane sizes are resized immediately, regardless of active / inactive status.
/// Only has an effect if the size has been defined earlier.
pub fn resize(w: u32, h: u32) -> Result<(), DivError> {
    state::exec_mut(|state| state.global_resize(w, h))
}
