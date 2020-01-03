#![macro_use] extern crate lazy_static;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use stdweb::web::Element;
use stdweb::traits::*;

pub mod error;
pub mod pane_handle;
pub mod global;
mod pane;
mod state;
mod storage;
mod style;

pub use error::*;
pub use pane_handle::*;
pub use global::*;
use storage::{PaneStorage, PaneHashMap};
use state::*;
use style::*;

/// Mounts the panes to the HTML body
pub fn init() -> Result<(), PanesError> {
    init_ex(None,(0,0),None)
}

/// Extended initialization function.
/// Mounts the panes as a child of the HTML element with the defined ID.
/// The specified dimensions restrict the area in which panes are visible.
/// # Example
/// ```
/// let width = 1280
/// let height = 720
/// panes::init_ex("panes-root", 0, 0, width, height);
/// ```
pub fn init_ex(
    id: Option<&str>,
    pos: (u32,u32),
    size: Option<(u32, u32)>
) 
-> Result<(), PanesError>  
{
    let mut state = get_mut()?;
    if state.is_some() {
        return Err(PanesError::AlreadyInitialized);
    }

    let root = get_root(id)?;
    *state = Some(GlobalState {
        root,
        nodes: PaneHashMap::default(),
        pos,
        size,
        zoom: (1.0, 1.0),
    });
    add_panes_styles_to_document()
}

fn get_root(id: Option<&str>) -> Result<Element, PanesError> {
    let document = stdweb::web::document();
    let element = 
    if id.is_some() {
        document.get_element_by_id(id.unwrap()).ok_or(PanesError::MissingRoot(id.unwrap().to_owned()))?
    } else {
        document.body().ok_or(PanesError::MissingBody)?.into()
    };
    Ok(element)
}

/// Creates a new pane at the defined position with the given HTML as content.
/// Use the returned PaneHandle to manipulate the pane.
pub fn new_pane(
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    html: &str
)
-> Result<PaneHandle, PanesError> 
{
    let mut state = get_mut()?;
    let css = "";
    let classes = "";
    let ph = state.as_mut()
        .ok_or(PanesError::NotInitialized)?
        .new_pane(x,y,w,h,html,css,classes)?;
    Ok(ph)
}

/// Creates a new pane at the defined position with the given HTML as content and with CSS classes and inline styles.
/// 
/// Most of the time, classes in combination with a style-sheet is the best way to style HTML.
/// But sometimes, it can be useful to add styles right on a pane.
/// 
/// This function has several generic parameters to maximize flexibility and allow for all combinations of &str and String.
/// When using empty iterators, sometimes the compiler gets irritated.
/// Use explicit type to help it.
/// # Example
/// ```
/// let html = "Some text";
/// let classes = ["my-class"];
/// let css: [(&str, &str);0] = [];
/// let pane = panes::new_styled_pane(
///     0,0,1000,1000,
///     html,
///     &classes,
///     &css,
/// ).unwrap();
/// ```
pub fn new_styled_pane<'a, C, CSS, S1, S2, S3>
(
    x: u32, 
    y: u32,
    w: u32,
    h: u32,
    html: &str,
    classes: C,
    css: CSS,
) 
-> Result<PaneHandle, PanesError> 
where
    C: IntoIterator<Item = &'a S1>,
    CSS: IntoIterator<Item = &'a(S2,S3)>,
    S1: AsRef<str> + 'a,
    S2: AsRef<str> + 'a,
    S3: AsRef<str> + 'a,
{
    let mut state = get_mut()?;

    let css_str = css.into_iter()
        .map(
            |(attr,val)|
            attr.as_ref().to_owned() + ": " + val.as_ref() + ";"
        )
        .collect::<Vec<_>>()
        .join(" ");

    let classes_str = classes.into_iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(" ");

    let ph = state.as_mut()
        .ok_or(PanesError::NotInitialized)?
        .new_pane(
            x,y,w,h,
            html,
            &classes_str,
            &css_str,
        )?;
    Ok(ph)
}