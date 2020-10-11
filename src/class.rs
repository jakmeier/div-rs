mod load;
pub use load::*;
use web_sys::HtmlElement;

use crate::ClassStorage;
use crate::PanesError;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub struct JsClassHandle {
    index: usize,
}
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct JsClass {
    name: String,
}
#[derive(Debug, Default)]
pub(crate) struct JsClassStorage {
    data: Vec<JsClass>,
}

impl ClassStorage for JsClassStorage {
    fn get(&self, class: JsClassHandle) -> &JsClass {
        &self.data[class.index]
    }
}

impl JsClassStorage {
    pub(crate) fn load(
        &mut self,
        classes: &[&str],
        src: &str,
    ) -> Result<impl std::future::Future<Output = Vec<JsClassHandle>>, PanesError> {
        let code = build_class_loading_module(classes, src);
        let mut out = vec![];
        for class_name in classes {
            let index = self.data.len();
            let class = JsClass {
                name: class_name.to_string(),
            };
            self.data.push(class);
            out.push(JsClassHandle { index })
        }
        let future = load_js_module(code)?;
        Ok(async {
            future.await;
            out
        })
    }
    pub(crate) fn preloaded(&mut self, name: &str) -> Option<JsClassHandle> {
        if let Some(class) = self.find_by_name(name) {
            return Some(class);
        }
        if svelte_component_exists(name) {
            let index = self.data.len();
            let class = JsClass {
                name: name.to_string(),
            };
            self.data.push(class);
            let class_handle = JsClassHandle { index };
            return Some(class_handle);
        }
        None
    }
    fn find_by_name(&self, name: &str) -> Option<JsClassHandle> {
        self.data
            .iter()
            .position(|ch| ch.name == name)
            .map(|index| JsClassHandle { index })
    }
}

impl JsClass {
    pub(crate) fn attach_new_instance(&self, node: &HtmlElement) {
        instantiate_svelte_component(&self.name, node);
    }
}
