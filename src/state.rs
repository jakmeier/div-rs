use crate::*;
pub(crate) struct GlobalState<PS, CS>
where
    PS: PaneStorage,
    CS: ClassStorage,
{
    pub(crate) root: web_sys::Element,
    pub(crate) pos: (i32, i32),
    pub(crate) size: Option<(u32, u32)>,
    pub(crate) zoom: (f32, f32),
    pub(crate) nodes: PS,
    pub(crate) classes: CS,
}
use std::thread_local;
thread_local! {
    static S_STATE: RwLock<Option<GlobalState<PaneHashMap, JsClassStorage>>> = RwLock::default();
}

// pub (crate) fn get<'a>() -> Result<RwLockReadGuard<'a, Option<GlobalState<PaneHashMap, JsClassStorage>>>, DivError>
// pub(crate) fn get(
// ) -> Result<RwLockReadGuard<'a, Option<GlobalState<PaneHashMap, JsClassStorage>>>, DivError> {
//     S_STATE.with(|state| state.read().map_err(|_e| DivError::Locked))
// }

pub(crate) fn set_state(
    new_state: GlobalState<PaneHashMap, JsClassStorage>,
) -> Result<(), DivError> {
    S_STATE.with(|state| {
        let mut state = state.write().map_err(|_e| DivError::Locked)?;
        if state.is_some() {
            return Err(DivError::AlreadyInitialized);
        }
        state.replace(new_state);
        Ok(())
    })
}

pub(crate) fn get_class(class_handle: JsClassHandle) -> Result<JsClass, DivError> {
    S_STATE.with(|state| {
        let state = state.read().map_err(|_e| DivError::Locked)?;
        let class = state
            .as_ref()
            .ok_or(DivError::NotInitialized)?
            .classes
            .get(class_handle);
        Ok(class.clone())
    })
}

pub(crate) fn exec<T, F>(f: F) -> Result<T, DivError>
where
    F: FnOnce(&GlobalState<PaneHashMap, JsClassStorage>) -> Result<T, DivError>,
{
    S_STATE.with(|state| {
        let state = state.read().map_err(|_e| DivError::Locked)?;
        f(state.as_ref().as_ref().ok_or(DivError::NotInitialized)?)
    })
}
pub(crate) fn exec_mut<T, F>(f: F) -> Result<T, DivError>
where
    F: FnOnce(&mut GlobalState<PaneHashMap, JsClassStorage>) -> Result<T, DivError>,
{
    S_STATE.with(|state| {
        let mut state = state.write().map_err(|_e| DivError::Locked)?;
        f(state.as_mut().as_mut().ok_or(DivError::NotInitialized)?)
    })
}
