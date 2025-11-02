use gpui::{hsla, rgb, Hsla};

#[derive(Clone)]
pub struct Theme {
    pub background: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: hsla(0.59, 0.13, 0.20, 0.75),
            text: rgb(0xabb2bf).into(),
            text_muted: hsla(0.59, 0.11, 0.55, 0.6),
            selection: hsla(0.59, 0.13, 0.31, 0.7),
            cursor: rgb(0x528bff).into(),
        }
    }
}
