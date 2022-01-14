pub mod application;
pub mod events;
pub mod message;
pub mod ui;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    x: f32,
    y: f32,
}
impl Eq for Size {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}
impl Eq for Position {}
