#![no_std]

use crate::menu;

#[derive(Debug)]
pub enum ScreenTypes {
    LoadScreenTypes,
    MenuScreenTypes(menu::menu_screen_state::MenuScreenTypesState),
}
