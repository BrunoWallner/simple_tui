#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StyleSheet {
    pub width: Length,
    pub height: Length,
    pub padding: Padding, 
}
impl Default for StyleSheet {
    fn default() -> Self {
        Self {
            width: Length::Relative(0.25),
            height: Length::Relative(0.25),
            padding: Padding {
                x_start: 1.0,
                x_end: 0.0,
                y_start: 1.0,
                y_end: 0.0,
            },
        }
    }
}
impl Eq for StyleSheet {}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    Absolute(f32),
    Relative(f32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Padding {
    pub x_start: f32,
    pub x_end: f32,
    pub y_start: f32,
    pub y_end: f32,
}

pub trait Style {
    fn get_style(&self) -> StyleSheet;
    fn set_style(&mut self, style: StyleSheet);
    fn apply_style(&mut self);
}
