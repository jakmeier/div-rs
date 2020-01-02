use crate::*;
use lazy_static::lazy_static;

pub (crate) struct GlobalState<PS> 
where PS: PaneStorage
{
    pub (crate) root: Element,
    pub (crate) pos: (u32, u32),
    pub (crate) size: Option<(u32, u32)>,
    pub (crate) zoom: (f32, f32),
    pub (crate) nodes: PS,
}

// This library is built on the premise that it will only run in a single thread of a browser.
// Until there is browser support for multiple threads that can access the same DOM, there 
// should be no reason to challenge this assumption. Therefore, we could use `static mut` here 
// and some unsafe code to access it and it would mostly be fine. However, recursive mutable access
// to shared state can still cause problems with state being mutated when we would not expect it to
// do so, which then becomes annoying to debug.
// Besides, the RwLock implementation for the web target is really light-weight (basically a single 
// boolean). Using it therefore does not add unnecessary costs but it keeps rustc happy and helpful.
// TLDR: Using an RwLock here is more idiomatic than not using it, even on a single-threaded machine.
lazy_static! {
    static ref S_STATE: RwLock<Option<GlobalState<PaneHashMap>>> = RwLock::default();
}

pub (crate) fn get<'a>() -> Result<RwLockReadGuard<'a, Option<GlobalState<PaneHashMap>>>, PanesError>
{
    S_STATE.read().map_err(|_e| PanesError::Locked)
}

pub (crate) fn get_mut<'a>() -> Result<RwLockWriteGuard<'a, Option<GlobalState<PaneHashMap>>>,PanesError>
{
    S_STATE.write().map_err(|_e| PanesError::Locked)
}