#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StyleSheet {
    pub width: Length,
    pub height: Length,
    pub padding: (f32, f32), 
}
impl Default for StyleSheet {
    fn default() -> Self {
        Self {
            width: Length::Relative(0.25),
            height: Length::Relative(0.25),
            padding: (1.0, 1.0),
        }
    }
}
impl Eq for StyleSheet {}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    Absolute(f32),
    Relative(f32),
}

pub trait Style {
    fn get_style(&self) -> StyleSheet;
    fn set_style(&mut self, style: StyleSheet);
    fn apply_style(&mut self);
}
