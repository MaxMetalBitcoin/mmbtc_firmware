#![no_std]

use crate::display_type;
use crate::mm_state_action::MMStateAction;
use crate::prompt_screen_state::PromptScreenState;
use crate::Screen;
use core::fmt::Error;
use heapless::Vec;

#[derive(Debug, Clone)]
pub struct ChooseNetworkScreen {
    pub prompt_screen_state: PromptScreenState,
}

impl ChooseNetworkScreen {
    pub fn init() -> ChooseNetworkScreen {
        let mut choices: Vec<&'static str, 20> = Vec::new();
        choices.push(" Main").unwrap();
        choices.push(" Testnet").unwrap();
        choices.push(" Signet").unwrap();

        Self {
            prompt_screen_state: PromptScreenState {
                prompt: "Choose your network:",
                choices: choices,
                hover_index: 0,
            },
        }
    }

    pub fn render(
        &self,
        mut display: display_type::DisplayType,
    ) -> Result<display_type::DisplayType, Error> {
        self.prompt_screen_state.render(display)
    }

    pub fn update_state(&mut self, action: MMStateAction) -> bool {
        if action == MMStateAction::Enter {
            false
        } else {
            self.prompt_screen_state.update_state(action)
        }
    }
}
