use crate::layouts::KeyboardLayout;

pub struct Context<'a> {
    pub depth: usize,
    pub indent_size: usize,
    pub keymap: bool,
    pub bindings: bool,
    pub layout: &'a KeyboardLayout,
}

impl<'a> Context<'a> {
    pub fn with_indent(&self, indent: usize) -> Self {
        Self { depth: indent, ..*self }
    }

    pub fn inc(&self, increment: usize) -> Self {
        Self { depth: self.depth + increment, ..*self }
    }

    pub fn dec(&self, decrement: usize) -> Self {
        Self { depth: self.depth - decrement, ..*self }
    }

    pub fn keymap(&self) -> Self {
        Self { keymap: true, ..*self }
    }

    pub fn bindings(&self) -> Self {
        Self { bindings: true, ..*self }
    }
}
