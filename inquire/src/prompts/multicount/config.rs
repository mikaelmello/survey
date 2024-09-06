use crate::MultiCount;

/// Configuration settings used in the execution of a MultiCountPrompt.
#[derive(Copy, Clone, Debug)]
pub struct MultiCountConfig {
    /// Whether to use vim-style keybindings.
    pub vim_mode: bool,
    /// Page size of the list of options.
    pub page_size: usize,
    /// Whether to keep the filter text when an option is selected.
    pub keep_filter: bool,
    /// Whether to reset the cursor to the first option on filter input change.
    pub reset_cursor: bool,
}

impl<T> From<&MultiCount<'_, T>> for MultiCountConfig {
    fn from(value: &MultiCount<'_, T>) -> Self {
        Self {
            vim_mode: value.vim_mode,
            page_size: value.page_size,
            keep_filter: value.keep_filter,
            reset_cursor: value.reset_cursor,
        }
    }
}
