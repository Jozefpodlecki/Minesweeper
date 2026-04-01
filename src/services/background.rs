use std::rc::Rc;

use web_sys::HtmlImageElement;


#[derive(Debug, Clone, PartialEq)]
pub struct DefaultBackground {
    inner: HtmlImageElement,
    src: Rc<str>
}

impl DefaultBackground {
    pub fn new(image: HtmlImageElement) -> Self {
        let src = image.src();

        Self {
            inner: image,
            src: src.into()
        }
    }

    pub fn src(&self) -> Rc<str> {
        self.src.clone()
    }

    pub fn set_opacity(&self) {
        let styles = self.inner.style();
        styles.set_property("opacity", "0").unwrap();
    }
}