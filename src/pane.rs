use crate::storage::PaneStorage;
use crate::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Internal representation of the state required to control a div in the browser.
///
/// The first name for the crate was pane and a PaneHandle was exposed to library users.
/// But a crate names pane already existed on crates.io and I decided div is a better name.
/// As a consequence, all API exposed functions should only refer to divs.
/// Internally, however, the name pane is still used.
#[derive(Debug)]
pub(crate) struct Pane {
    node: HtmlElement,
    displayed: bool,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl<PS: PaneStorage, CS: ClassStorage> GlobalState<PS, CS> {
    /// Creates a new pane from specified html and shows it
    pub(crate) fn new_pane(
        &mut self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        html: &str,
        classes: &str,
        css: &str,
    ) -> Result<DivHandle, DivError> {
        let window = web_sys::window().ok_or(DivError::MissingWindow)?;
        let doc = window.document().ok_or(DivError::MissingDocument)?;

        let x = self.pos.0 + x;
        let y = self.pos.1 + y;

        let node: HtmlElement = doc
            .create_element("div")?
            .dyn_into()
            .map_err(|_| DivError::JsCastError)?;
        node.set_class_name(&("div-rs".to_owned() + classes));
        node.set_inner_html(html);
        node.set_attribute("style", css)?;

        self.root.append_with_node_1(&node.clone().into())?;
        let vnode = Pane {
            node,
            displayed: true,
            x,
            y,
            w,
            h,
        };
        vnode.redraw(self.pos, self.zoom)?;

        let ph = self.nodes.insert(vnode);
        Ok(ph)
    }
    pub(crate) fn hide_pane(&mut self, p: &DivHandle) -> Result<(), DivError> {
        let v = &self.nodes.get_mut(p)?;
        if v.displayed {
            let old_node = self
                .root
                .remove_child(&v.node)
                .map_err(|_e| DivError::MissingChild)?;
            let entry = self.nodes.get_mut(p).unwrap();
            entry.node = old_node.dyn_into().map_err(|_| DivError::JsCastError)?;
            entry.displayed = false;
        }
        Ok(())
    }
    pub(crate) fn show_pane(&mut self, p: &DivHandle) -> Result<(), DivError> {
        let mut v = self.nodes.get_mut(&p)?;
        if !v.displayed {
            self.root.append_child(&v.node)?;
            v.displayed = true;
        }
        Ok(())
    }
    pub(crate) fn delete_pane(&mut self, p: &DivHandle) -> Result<(), DivError> {
        // This removes the node from the DOM
        self.hide_pane(p)?;
        // This deletes all references for GC
        self.nodes.remove(p)?;
        Ok(())
    }
    pub(crate) fn get_node(&self, p: &DivHandle) -> Result<&HtmlElement, DivError> {
        let v = self.nodes.get(&p)?;
        Ok(&v.node)
    }
    #[inline(always)]
    pub(crate) fn update_pane(
        &mut self,
        pane_handle: &DivHandle,
        x: Option<u32>,
        y: Option<u32>,
        w: Option<u32>,
        h: Option<u32>,
    ) -> Result<(), DivError> {
        let mut v = self.nodes.get_mut(&pane_handle)?;
        v.x = x.unwrap_or(v.x);
        v.y = y.unwrap_or(v.y);
        v.w = w.unwrap_or(v.w);
        v.h = h.unwrap_or(v.h);
        v.redraw(self.pos, self.zoom)?;
        Ok(())
    }
    pub(crate) fn global_reposition(&mut self, x: u32, y: u32) -> Result<(), DivError> {
        self.pos = (x, y);
        self.nodes.for_each(&|pane: &mut Pane| {
            let el = pane.get_element()?;
            let x = x + pane.x;
            let y = y + pane.y;
            let style = el.style();
            style.set_property("left", &format!("{}px", x))?;
            style.set_property("top", &format!("{}px", y))?;
            // js! { @(no_return)
            //     @{&el}.style.left=@{x} + "px";
            //     @{&el}.style.top=@{y} + "px";
            // };
            Ok(())
        })
    }
    pub(crate) fn global_resize(&mut self, w: u32, h: u32) -> Result<(), DivError> {
        if let Some((width, height)) = self.size {
            let fx = w as f32 / width as f32;
            let fy = h as f32 / height as f32;
            self.zoom = (fx, fy);
            let zoom = self.zoom;
            let pos = self.pos;
            self.nodes.for_each(&|p| p.redraw(pos, zoom))
        } else {
            Err(DivError::UndefinedSize)
        }
    }
}

impl Pane {
    pub(crate) fn get_element(&self) -> Result<&HtmlElement, DivError> {
        Ok(&self.node)
    }
    pub(crate) fn redraw(&self, (x, y): (u32, u32), (fx, fy): (f32, f32)) -> Result<(), DivError> {
        let x = x + (fx * self.x as f32) as u32;
        let y = y + (fy * self.y as f32) as u32;
        let w = (fx * self.w as f32) as u32;
        let h = (fy * self.h as f32) as u32;

        let style = self.node.style();
        style.set_property("left", &format!("{}px", x))?;
        style.set_property("top", &format!("{}px", y))?;
        style.set_property("width", &format!("{}px", w))?;
        style.set_property("height", &format!("{}px", h))?;

        Ok(())
    }
    pub(crate) fn set_css(&self, property: &str, value: &str) -> Result<(), DivError> {
        self.node.style().set_property(property, value)?;
        Ok(())
    }
    pub(crate) fn add_class(&self, css_class: &str) -> Result<(), DivError> {
        self.node.class_list().add_1(css_class)?;
        Ok(())
    }
    pub(crate) fn remove_class(&self, css_class: &str) -> Result<(), DivError> {
        self.node.class_list().remove_1(css_class)?;
        Ok(())
    }
}
