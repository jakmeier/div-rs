use crate::DivError;
use std::sync::atomic::{AtomicI32, Ordering};
use std::task::Poll;
use std::{pin::Pin, task::Context};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlScriptElement};

#[wasm_bindgen(module = "/src/class/class_loader.js")]
extern "C" {
    pub(super) fn instantiate_svelte_component(a: &str, node: &HtmlElement);
    fn loading_progress() -> i32;
    pub(super) fn svelte_component_exists(name: &str) -> bool;
    pub fn init_div_rs();
}

static LOADED: AtomicI32 = AtomicI32::new(0);

pub(super) fn build_class_loading_module(classes: &[&str], src: &str) -> String {
    let mut code = format!(
        "import {{{}}} from '{}';\nwindow.__div_rs = window.__div_rs || {{svcom: {{}}, loaded: 0}};",
        classes.join(", "),
        src
    );
    for class_name in classes {
        code += "\nwindow.__div_rs.svcom.";
        code += class_name;
        code += " = classes.";
        code += class_name;
        code += ";";
    }
    code
}

/// **Experimental: This API is experimental and my not be included in later versions**
/// Asynchronously loads a JS module by appending a script tag to the head with th e provided string as content.
/// Poll the future until it resolves to know when the script has been loaded for sure.
/// In contrast to the more conventional Future design, the JS module will be loaded even if the Future is not polled.
/// The Future only checks if it has already finished.
pub fn load_js_module(mut code: String) -> Result<PendingScript, DivError> {
    let window = web_sys::window().ok_or(DivError::MissingWindow)?;
    let doc = window.document().ok_or(DivError::MissingDocument)?;
    let script: HtmlScriptElement = doc
        .create_element("script")?
        .dyn_into()
        .map_err(|_| DivError::JsCastError)?;
    script.set_attribute("type", "module")?;
    code += "\nwindow.__div_rs = window.__div_rs || {};";
    code += "\nwindow.__div_rs.loaded = (window.div.loaded || 0) + 1;";
    script.set_text(&code)?;

    let head = doc.head().ok_or(DivError::MissingHead)?;
    head.append_child(&script)?;

    // At this point, everything is set up for the event loop to load our script in the future.
    // We can poll this simple future to check when that is done.
    let ticket = LOADED.fetch_add(1, Ordering::Relaxed);
    Ok(PendingScript { ticket })
}

pub struct PendingScript {
    ticket: i32,
}
impl std::future::Future for PendingScript {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let loaded = loading_progress();
        if loaded > self.ticket {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
