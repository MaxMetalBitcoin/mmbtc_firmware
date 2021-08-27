#![no_std]

#[derive(Debug, PartialEq, Eq)]
pub enum MMStateAction {
    Up,
    Down,
    Left,
    Right,
    Enter,
}
