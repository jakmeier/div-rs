use crate::pane::Pane;
use crate::*;
use std::collections::HashMap;

/// A trait for data structures which store a div and assign unique DivHandle to them
pub(crate) trait PaneStorage {
    fn insert(&mut self, p: Pane) -> DivHandle;
    fn remove(&mut self, p: &DivHandle) -> Result<Pane, DivError>;
    fn get(&self, p: &DivHandle) -> Result<&Pane, DivError>;
    fn get_mut(&mut self, p: &DivHandle) -> Result<&mut Pane, DivError>;
    fn for_each<F>(&mut self, f: &F) -> Result<(), DivError>
    where
        F: Fn(&mut Pane) -> Result<(), DivError>;
}
/// A trait for data structures which store information about JS classes loaded in
pub(crate) trait ClassStorage {
    fn get(&self, class: JsClassHandle) -> &JsClass;
}

/// At the moment, uses a std HashMap internally.
#[derive(Debug, Default)]
pub(crate) struct PaneHashMap {
    data: HashMap<usize, Pane>,
    next_idx: usize,
}

impl PaneStorage for PaneHashMap {
    fn insert(&mut self, p: Pane) -> DivHandle {
        let i = self.next_id();
        self.data.insert(i, p);
        DivHandle(i)
    }
    fn remove(&mut self, p: &DivHandle) -> Result<Pane, DivError> {
        self.data
            .remove(&p.0)
            .ok_or_else(|| index_error(&p, self.next_idx))
    }
    fn get(&self, p: &DivHandle) -> Result<&Pane, DivError> {
        self.data
            .get(&p.0)
            .ok_or_else(|| index_error(p, self.next_idx))
    }
    fn get_mut(&mut self, p: &DivHandle) -> Result<&mut Pane, DivError> {
        let idx = self.next_idx;
        self.data.get_mut(&p.0).ok_or_else(|| index_error(p, idx))
    }
    fn for_each<F>(&mut self, f: &F) -> Result<(), DivError>
    where
        F: Fn(&mut Pane) -> Result<(), DivError>,
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
        self.next_idx = i + 1;
        i
    }
}

fn index_error(p: &DivHandle, max_idx: usize) -> DivError {
    if max_idx > p.0 {
        DivError::UseAfterDelete
    } else {
        DivError::NotAllocated
    }
}
