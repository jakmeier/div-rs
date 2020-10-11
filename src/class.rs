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
}

impl JsClass {
    pub fn attach_new_instance(&self, node: &HtmlElement) {
        instantiate_class(&self.name, node);
    }
}
