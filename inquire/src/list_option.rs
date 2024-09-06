//! Utilities used to wrap user selections in [Select](crate::Select) and
//! [`MultiSelect`](crate::MultiSelect) prompts.

use std::fmt;

/// Represents a selection made by the user when prompted to select one or several
/// options among those presented.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListOption<T> {
    /// Index of the selected option relative to the original (full) list passed to the prompt.
    pub index: usize,

    /// Value of the selected option.
    pub value: T,
}

impl<T> ListOption<T> {
    /// Constructor for `ListOption`.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the option.
    /// * `value` - String value of the option
    ///
    /// # Examples
    ///
    /// ```
    /// use inquire::list_option::ListOption;
    ///
    /// let answer = ListOption::new(0, "a");
    /// ```
    pub fn new(index: usize, value: T) -> Self {
        Self { index, value }
    }

    /// Converts from `&ListOption<T>` to `ListOption<&T>`.
    pub fn as_ref(&self) -> ListOption<&T> {
        ListOption::new(self.index, &self.value)
    }
}

impl<T> fmt::Display for ListOption<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Represents a selection made by a user alongside a count of said options, when prompted
/// to select a count of each of several presented options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountedListOption<T> {
    /// The count of the option selected.
    pub count: u32,
    /// The option selected.
    pub list_option: ListOption<T>,
}

impl<T> CountedListOption<T> {
    /// Constructor for `CountedListOption`.
    ///
    /// # Arguments
    ///
    /// * `count` - Count of elements chosen.
    /// * `list_option` - A ListOption representing the choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use inquire::list_option::{ListOption, CountedListOption};
    ///
    /// let answer = CountedListOption::new(0, ListOption::new(0, "a"));
    /// ```
    pub fn new(count: u32, list_option: ListOption<T>) -> Self {
        Self { count, list_option }
    }

    /// Converts from `&CountedListOption<T>` to `CountedListOption<&T>`.
    pub fn as_ref(&self) -> CountedListOption<&T> {
        CountedListOption {
            count: self.count,
            list_option: self.list_option.as_ref(),
        }
    }
}
