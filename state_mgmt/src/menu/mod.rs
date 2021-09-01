#![no_std]

use core::fmt::Error;

pub mod choose_network_menu;
pub mod prompt_screen_state;
pub use crate::display_type;
pub use crate::mm_state_action;

#[derive(Debug)]
pub enum MenuTypes {
    ChooseNetworkMenuType(choose_network_menu::ChooseNetworkMenu),
}
