use stdweb::web::Node;
use crate::*;
use crate::storage::PaneStorage;
use stdweb::unstable::TryInto;
use stdweb::js;

/// Internal representation of a Pane 
#[derive(Debug)]
pub (crate) struct Pane {
    node: Node,
    displayed: bool,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}


impl<PS: PaneStorage> GlobalState<PS> {

    /// Creates a new pane from specified html and shows it
    pub (crate) fn new_pane(&mut self, x: u32, y: u32, w: u32, h: u32, html: &str) -> Result<PaneHandle, PanesError> {
        let x = self.x + x;
        let y = self.y + y;
        let wrapped_html = format!(r#"<div class="pane" style="left: {}px; top: {}px; width: {}px; height: {}px">{}</div>"#, x, y, w, h, html);
        self.root.append_html(&wrapped_html)
            .map_err(|e|PanesError::BrowserError(Box::new(e)))?;
        let node = self.root.last_child().ok_or(PanesError::MissingChild)?;
        let vnode = Pane {node, displayed: true, x, y, w, h};
        let ph = self.nodes.insert(vnode);
        Ok(ph)
    }
    pub (crate) fn hide_pane(&mut self, p: &PaneHandle) -> Result<(), PanesError> {
        let v = &self.nodes.get_mut(p)?;
        if v.displayed {
            let old_node = self.root.remove_child(&v.node).map_err(|_e| PanesError::MissingChild)?;
            let entry = self.nodes.get_mut(p).unwrap();
            entry.node = old_node;
            entry.displayed = false;
        }
        Ok(())
    }
    pub (crate) fn show_pane(&mut self, p: &PaneHandle) -> Result<(), PanesError> {
        let mut v = self.nodes.get_mut(&p)?;
        if !v.displayed {
            self.root.append_child(&v.node);
            v.displayed = true;
        }
        Ok(())
    }
    pub (crate) fn delete_pane(&mut self, p: PaneHandle) -> Result<(), PanesError> {
        // This removes the node from the DOM
        self.hide_pane(&p)?;
        // This deletes all references for GC
        self.nodes.remove(p)?;
        Ok(())
    }
    pub (crate) fn get_node(&self, p: &PaneHandle) -> Result<&Node, PanesError> {
        let v = self.nodes.get(&p)?;
        Ok(&v.node)
    }

    pub (crate) fn reposition_panes(&mut self, x: u32, y: u32) -> Result<(), PanesError> {
        self.x = x;
        self.y = y;
        self.nodes.for_each(
            &|pane: &mut Pane| {
                let el = pane.get_element()?;
                let x = x + pane.x;
                let y = y + pane.y;
                js! { @(no_return)
                    @{&el}.style.left=@{x} + "px";
                    @{&el}.style.top=@{y} + "px";
                };
                Ok(())
            }
        )
    }
    pub (crate) fn resize_panes(&mut self, w: u32, h: u32) -> Result<(), PanesError> {
        if let Some((width, height)) = self.size {
            let fx = w as f32 / width as f32;
            let fy = h as f32 / height as f32;
            let x = self.x;
            let y = self.y;
            self.nodes.for_each(
                &|pane: &mut Pane| {
                    let el = pane.get_element()?;
                    let x = x + (fx * pane.x as f32) as u32;
                    let y = y + (fy * pane.y as f32) as u32;
                    let w = (fx * pane.w as f32) as u32;
                    let h = (fy * pane.h as f32) as u32;
                    js! { @(no_return)
                        @{&el}.style.left=@{x} + "px";
                        @{&el}.style.top=@{y} + "px";
                        @{&el}.style.width=@{w} + "px";
                        @{&el}.style.height=@{h} + "px";
                    };
                    Ok(())
                }
            )
        } else {
            Err(PanesError::UndefinedSize)
        }
    }
}

impl Pane {
    pub (crate) fn get_element(&self) -> Result<Element, PanesError> {
        self.node.clone().try_into().map_err(
            |_e| PanesError::MissingChild
        )
    }
}