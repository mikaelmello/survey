use crate::{
    ui::{Key, KeyModifiers},
    InnerAction, InputAction,
};

use super::config::MultiCountConfig;

/// Set of actions for a MultiCountPrompt.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MultiCountPromptAction {
    /// Action on the value text input handler.
    FilterInput(InputAction),
    /// Moves the cursor to the option above.
    MoveUp,
    /// Moves the cursor to the option below.
    MoveDown,
    /// Moves the cursor to the page above.
    PageUp,
    /// Moves the cursor to the page below.
    PageDown,
    /// Moves the cursor to the start of the list.
    MoveToStart,
    /// Moves the cursor to the end of the list.
    MoveToEnd,
    /// Toggles the selection of the current option.
    SetCountCurrentOption(u32),
    /// Increments the current selection by one
    Increment,
    /// Decrements the current selection by one
    Decrement,
    /// Increments the current selection by the given amount
    MultiIncrement(u32),
    /// Decrements the current selection by the given amount
    MultiDecrement(u32),
    /// Clears counts for all options.
    ClearSelections,
}

impl InnerAction for MultiCountPromptAction {
    type Config = MultiCountConfig;

    fn from_key(key: Key, config: &MultiCountConfig) -> Option<Self> {
        if config.vim_mode {
            let action = match key {
                Key::Char('k', KeyModifiers::NONE) => Some(Self::MoveUp),
                Key::Char('j', KeyModifiers::NONE) => Some(Self::MoveDown),
                Key::Char('+', KeyModifiers::NONE) => Some(Self::Increment),
                Key::Char('-', KeyModifiers::NONE) => Some(Self::Decrement),
                _ => None,
            };
            if action.is_some() {
                return action;
            }
        }

        let action = match key {
            Key::Up(KeyModifiers::NONE) | Key::Char('p', KeyModifiers::CONTROL) => Self::MoveUp,

            Key::PageUp(_) => Self::PageUp,
            Key::Home => Self::MoveToStart,

            Key::Down(KeyModifiers::NONE) | Key::Char('n', KeyModifiers::CONTROL) => Self::MoveDown,
            Key::PageDown(_) => Self::PageDown,
            Key::End => Self::MoveToEnd,

            Key::Right(KeyModifiers::NONE) => Self::Increment,
            Key::Left(KeyModifiers::NONE) => Self::Decrement,
            Key::Right(KeyModifiers::SHIFT) => Self::MultiIncrement(10),
            Key::Left(KeyModifiers::SHIFT) => Self::MultiDecrement(10),
            key => match InputAction::from_key(key, &()) {
                Some(action) => Self::FilterInput(action),
                None => return None,
            },
        };
        Some(action)
    }
}
