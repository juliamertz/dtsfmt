mod adv360;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum KeyboardLayoutType {
    /// Kinesis Advantage 360
    #[clap(name = "kinesis:adv360")]
    Adv360,
}

pub struct KeyboardLayout {
    pub bindings: Vec<usize>,
    pub breakpoints: Vec<usize>,
}

pub fn get_layout(layout_type: KeyboardLayoutType) -> KeyboardLayout {
    match layout_type {
        KeyboardLayoutType::Adv360 => adv360::get_layout(),
    }
}