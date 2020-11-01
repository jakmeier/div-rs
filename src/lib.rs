use std::{future::Future, sync::RwLock};
use web_sys::Element;

mod class;
pub mod error;
pub mod global;
mod pane;
pub mod pane_handle;
mod state;
mod storage;
mod style;
mod utils;

pub use class::*;
pub use error::*;
pub use global::*;
pub use pane_handle::*;
pub use utils::doc;
use state::*;
use storage::{ClassStorage, PaneHashMap, PaneStorage};
use style::*;

/// Mounts the div to the HTML body
pub fn init() -> Result<(), DivError> {
    init_ex(None, (0, 0), None)
}

/// Mounts the div to a element with the given id
pub fn init_to(id: &str) -> Result<(), DivError> {
    init_ex(Some(id), (0, 0), None)
}

/// Extended initialization function.
/// Mounts the div as a child of the HTML element with the defined ID.
/// The specified dimensions restrict the area in which div are visible.
/// # Example
/// ```
/// let width = 1280;
/// let height = 720;
/// div::init_ex(Some("div-root"), (0, 0), Some((width, height)));
/// ```
pub fn init_ex(
    id: Option<&str>,
    pos: (u32, u32),
    size: Option<(u32, u32)>,
) -> Result<(), DivError> {
    let root = get_root(id)?;
    state::set_state(GlobalState {
        root,
        nodes: PaneHashMap::default(),
        pos,
        size,
        zoom: (1.0, 1.0),
        classes: JsClassStorage::default(),
    })?;
    add_div_styles_to_document()?;
    init_div_rs();
    Ok(())
}

fn get_root(id: Option<&str>) -> Result<Element, DivError> {
    let element = if id.is_some() {
        doc()?
            .get_element_by_id(id.unwrap())
            .ok_or(DivError::MissingRoot(id.unwrap().to_owned()))?
    } else {
        doc()?.body().ok_or(DivError::MissingBody)?.into()
    };
    Ok(element)
}

/// Creates a new pane at the defined position with the given HTML as content.
/// Use the returned PaneHandle to manipulate the pane.
pub fn new_pane(x: u32, y: u32, w: u32, h: u32, html: &str) -> Result<PaneHandle, DivError> {
    let css = "";
    let classes = "";
    state::exec_mut(|state| state.new_pane(x, y, w, h, html, css, classes))
}

/// Creates a new pane at the defined position with the given HTML as content and with CSS classes and inline styles.
///
/// Traditionally on the web, classes in combination with a style-sheet are the best way to apply CSS to HTML.
/// But sometimes, it can also be useful to add styles right on the top HTML element of a pane.
/// With this function, both options are open and can even be combined.
///
/// This function has several generic parameters to maximize flexibility and allow for all combinations of &str and String.
/// When using empty iterators, sometimes the compiler gets irritated.
/// Use explicit type to help it.
/// # Example
/// ```
/// let html = "Some text";
/// let classes = ["my-class"];
/// let css: [(&str, &str);0] = [];
/// let pane = div::new_styled_pane(
///     0,0,1000,1000,
///     html,
///     &classes,
///     &css,
/// ).unwrap();
/// ```
pub fn new_styled_pane<'a, C, CSS, S1, S2, S3>(
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    html: &str,
    classes: C,
    css: CSS,
) -> Result<PaneHandle, DivError>
where
    C: IntoIterator<Item = &'a S1>,
    CSS: IntoIterator<Item = &'a (S2, S3)>,
    S1: AsRef<str> + 'a,
    S2: AsRef<str> + 'a,
    S3: AsRef<str> + 'a,
{
    let css_str = css
        .into_iter()
        .map(|(attr, val)| attr.as_ref().to_owned() + ": " + val.as_ref() + ";")
        .collect::<Vec<_>>()
        .join(" ");

    let classes_str = classes
        .into_iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(" ");

    state::exec_mut(|state| state.new_pane(x, y, w, h, html, &classes_str, &css_str))
}

/// **Experimental: This API is experimental and my not be included in later versions**
/// Load a class named `name` from a JS file accessible at `src`.
///
/// Returns a Future because the script is loaded asynchronously.
/// That future will have to be handled in one way or another.
/// The most direct way would be to use `wasm_bindgen_futures::spawn_local`
/// ## Example
/// ```rust
/// const X: u32 = 100;
/// const Y: u32 = 100;
/// const W: u32 = 500;
/// const H: u32 = 500;
/// let future = async {
///     let class = div::load_js_class("Test", "./Test.js").unwrap().await;
///     div::from_js_class(X, Y, W, H, class).unwrap();
/// };
/// wasm_bindgen_futures::spawn_local(future);
/// ```
pub fn load_js_class(
    name: &str,
    src: &str,
) -> Result<impl Future<Output = JsClassHandle>, DivError> {
    let classes = load_js_classes(&[name], src)?;
    Ok(async { classes.await[0] })
}

/// **Experimental: This API is experimental and my not be included in later versions**
/// Attempts to load a JS module by its source path and loads the classes exported by it, as named by the classes parameter.
/// Usage is equivalent to `load_js_class`.
pub fn load_js_classes(
    classes: &[&str],
    src: &str,
) -> Result<impl Future<Output = Vec<JsClassHandle>>, DivError> {
    let future = state::exec_mut(|state| state.classes.load(classes, src))?;
    Ok(future)
}

/// Creates a new pane and fills it with a JS class.
pub fn from_js_class(
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    class_handle: JsClassHandle,
) -> Result<PaneHandle, DivError> {
    let ph = new_pane(x, y, w, h, "")?;
    let node = ph.parent_element()?;
    let class = state::get_class(class_handle)?;
    class.attach_new_instance(&node);
    Ok(ph)
}

impl JsClass {
    /// Load a JS class that has already been registered, usually by JS code.
    /// Return None if no such class has been registered.
    pub fn preregistered(name: &str) -> Option<JsClassHandle> {
        state::exec_mut(|state| Ok(state.classes.preloaded(name))).unwrap()
    }
}
