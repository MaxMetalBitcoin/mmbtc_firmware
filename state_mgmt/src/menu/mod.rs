#![no_std]

use core::fmt::Error;

pub mod choose_network_menu;
pub mod prompt_screen_state;
pub use crate::display_type;
pub use crate::mm_state_action;

pub trait PromptScreenTrait {
    fn init() -> Self;

    fn render(
        &self,
        display: display_type::DisplayType,
    ) -> Result<display_type::DisplayType, Error>;

    fn update_state(&mut self, action: mm_state_action::MMStateAction) -> bool;
}
