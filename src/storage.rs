use std::collections::HashMap;
use crate::*;
use crate::pane::Pane;

/// A trait for data structures which store a panes and assign unique PaneHandle to them
pub (crate) trait PaneStorage {
    fn insert(&mut self, p: Pane) -> PaneHandle;
    fn remove(&mut self, p: &PaneHandle) -> Result<Pane, PanesError>;
    fn get(&self, p: &PaneHandle) -> Result<&Pane, PanesError>;
    fn get_mut(&mut self, p: &PaneHandle) -> Result<&mut Pane, PanesError>;
    fn for_each<F>(&mut self, f: &F) -> Result<(), PanesError> 
        where F: Fn(&mut Pane)-> Result<(), PanesError>;
}

/// At the moment, uses a std HashMap internally.
#[derive(Debug,Default)]
pub (crate) struct PaneHashMap {
    data: HashMap<usize, Pane>,
    next_idx: usize,
}

impl PaneStorage for PaneHashMap {
    fn insert(&mut self, p: Pane) -> PaneHandle {
        let i = self.next_id();
        self.data.insert(i,p);
        PaneHandle(i)
    }
    fn remove(&mut self, p: &PaneHandle) -> Result<Pane, PanesError> {
        self.data.remove(&p.0).ok_or_else(
            || index_error(&p, self.next_idx)
        )
    }
    fn get(&self, p: &PaneHandle) -> Result<&Pane, PanesError> {
        self.data.get(&p.0)
            .ok_or_else(
                || index_error(p, self.next_idx)
            )
    }
    fn get_mut(&mut self, p: &PaneHandle) -> Result<&mut Pane, PanesError> {
        let idx = self.next_idx;
        self.data.get_mut(&p.0).ok_or_else(
            || index_error(p, idx)
        )
    }
    fn for_each<F>(&mut self, f: &F) -> Result<(), PanesError> 
    where F: Fn(&mut Pane)-> Result<(), PanesError>
    {
        for pane in self.data.values_mut() {
            f(pane)?;
        }
        Ok(())
    }

}

impl PaneHashMap {
    fn next_id(&mut self) -> usize {
        let i = self.next_idx;
        self.next_idx = i+1;
        i
    }
}

fn index_error(p: &PaneHandle, max_idx: usize) -> PanesError {
    if max_idx > p.0 {
        PanesError::UseAfterDelete
    } else {
        PanesError::NotAllocated
    }
}