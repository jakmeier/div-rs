use stdweb::web::Node;
use crate::*;
use crate::storage::PaneStorage;

/// Internal representation of a Pane 
#[derive(Debug)]
pub (crate) struct Pane {
    node: Node,
    displayed: bool,
}


impl<PS: PaneStorage> GlobalState<PS> {

    /// Creates a new pane from specified html and shows it
    pub (crate) fn new_pane(&mut self, x: u32, y: u32, w: u32, h: u32, html: &str) -> Result<PaneHandle, PanesError> {
        let x = self.x + x;
        let y = self.y + y;
        let wrapped_html = format!(r#"<span class="on-canvas" style="left: {}px; top: {}px; width: {}px; height: {}px">{}</span>"#, x, y, w, h, html);
        self.root.append_html(&wrapped_html)
            .map_err(|e|PanesError::BrowserError(Box::new(e)))?;
        let node = self.root.last_child().ok_or(PanesError::MissingChild)?;
        let vnode = Pane {node, displayed: true};
        let ph = self.nodes.insert(vnode);
        Ok(ph)
    }
    pub (crate) fn hide_pane(&mut self, p: &PaneHandle) -> Result<(), PanesError> {
        let v = &self.nodes.get_mut(p)?;
        if v.displayed {
            let old_node = self.root.remove_child(&v.node).map_err(|_e| PanesError::MissingChild)?;
            *self.nodes.get_mut(p).unwrap() = Pane { node: old_node, displayed: false };
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
}