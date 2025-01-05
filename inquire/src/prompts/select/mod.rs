mod action;
mod config;
mod prompt;
#[cfg(test)]
#[cfg(feature = "crossterm")]
mod test;

pub use action::*;
use std::fmt::Display;

use crate::{
    config::get_configuration,
    error::{InquireError, InquireResult},
    formatter::OptionFormatter,
    list_option::ListOption,
    prompts::prompt::Prompt,
    terminal::get_default_terminal,
    type_aliases::Scorer,
    ui::{Backend, RenderConfig, SelectBackend},
};

use self::prompt::SelectPrompt;

#[cfg(feature = "fuzzy")]
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
#[cfg(feature = "fuzzy")]
use once_cell::sync::Lazy;
#[cfg(feature = "fuzzy")]
static DEFAULT_MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().ignore_case());
/// Prompt suitable for when you need the user to select one option among many.
///
/// The user can select and submit the current highlighted option by pressing enter.
///
/// This prompt requires a prompt message and a **non-empty** `Vec` of options to be displayed to the user. The options can be of any type as long as they implement the `Display` trait. It is required that the `Vec` is moved to the prompt, as the prompt will return the selected option (`Vec` element) after the user submits.
/// - If the list is empty, the prompt operation will fail with an `InquireError::InvalidConfiguration` error.
///
/// This prompt does not support custom validators because of its nature. A submission always selects exactly one of the options. If this option was not supposed to be selected or is invalid in some way, it probably should not be included in the options list.
///
/// The options are paginated in order to provide a smooth experience to the user, with the default page size being 7. The user can move from the options and the pages will be updated accordingly, including moving from the last to the first options (or vice-versa).
///
/// Like all others, this prompt also allows you to customize several aspects of it:
///
/// - **Prompt message**: Required when creating the prompt.
/// - **Options list**: Options displayed to the user. Must be **non-empty**.
/// - **Starting cursor**: Index of the cursor when the prompt is first rendered. Default is 0 (first option). If the index is out-of-range of the option list, the prompt will fail with an [`InquireError::InvalidConfiguration`] error.
/// - **Starting filter input**: Sets the initial value of the filter section of the prompt.
/// - **Help message**: Message displayed at the line below the prompt.
/// - **Formatter**: Custom formatter in case you need to pre-process the user input before showing it as the final answer.
///   - Prints the selected option string value by default.
/// - **Page size**: Number of options displayed at once, 7 by default.
/// - **Display option indexes**: On long lists, it might be helpful to display the indexes of the options to the user. Via the `RenderConfig`, you can set the display mode of the indexes as a prefix of an option. The default configuration is `None`, to not render any index when displaying the options.
/// - **Scorer function**: Function that defines the order of options and if displayed as all.
///
/// # Example
///
/// ```no_run
/// use inquire::{error::InquireError, Select};
///
/// let options: Vec<&str> = vec!["Banana", "Apple", "Strawberry", "Grapes",
///     "Lemon", "Tangerine", "Watermelon", "Orange", "Pear", "Avocado", "Pineapple",
/// ];
///
/// let ans: Result<&str, InquireError> = Select::new("What's your favorite fruit?", options).prompt();
///
/// match ans {
///     Ok(choice) => println!("{}! That's mine too!", choice),
///     Err(_) => println!("There was an error, please try again"),
/// }
/// ```
///
/// [`InquireError::InvalidConfiguration`]: crate::error::InquireError::InvalidConfiguration
#[derive(Clone)]
pub struct Select<'a, T> {
    /// Message to be presented to the user.
    pub message: &'a str,

    /// Options displayed to the user.
    pub options: Vec<T>,

    /// Help message to be presented to the user.
    pub help_message: Option<&'a str>,

    /// Page size of the options displayed to the user.
    pub page_size: usize,

    /// Whether vim mode is enabled. When enabled, the user can
    /// navigate through the options using hjkl.
    pub vim_mode: bool,

    /// Starting cursor index of the selection.
    pub starting_cursor: usize,

    /// Starting filter input
    pub starting_filter_input: Option<&'a str>,

    /// Reset cursor position to first option on filter input change.
    /// Defaults to true.
    pub reset_cursor: bool,

    /// Whether to allow the option list to be filtered by user input or not.
    ///
    /// Defaults to true.
    pub filter_input_enabled: bool,

    /// Function called with the current user input to score the provided
    /// options.
    pub scorer: Scorer<'a, T>,

    /// Function that formats the user input and presents it to the user as the final rendering of the prompt.
    pub formatter: OptionFormatter<'a, T>,

    /// RenderConfig to apply to the rendered interface.
    ///
    /// Note: The default render config considers if the NO_COLOR environment variable
    /// is set to decide whether to render the colored config or the empty one.
    ///
    /// When overriding the config in a prompt, NO_COLOR is no longer considered and your
    /// config is treated as the only source of truth. If you want to customize colors
    /// and still support NO_COLOR, you will have to do this on your end.
    pub render_config: RenderConfig<'a>,
}

impl<'a, T> Select<'a, T>
where
    T: Display,
{
    /// String formatter used by default in [Select](crate::Select) prompts.
    /// Simply prints the string value contained in the selected option.
    ///
    /// # Examples
    ///
    /// ```
    /// use inquire::list_option::ListOption;
    /// use inquire::Select;
    ///
    /// let formatter = Select::<&str>::DEFAULT_FORMATTER;
    /// assert_eq!(String::from("First option"), formatter(ListOption::new(0, &"First option")));
    /// assert_eq!(String::from("First option"), formatter(ListOption::new(11, &"First option")));
    /// ```
    pub const DEFAULT_FORMATTER: OptionFormatter<'a, T> = &|ans| ans.to_string();

    /// Default scoring function, which will create a score for the current option using the input value.
    /// The return will be sorted in Descending order, leaving options with None as a score.
    ///
    /// # Examples
    ///
    /// ```
    /// use inquire::Select;
    ///
    /// let scorer = Select::<&str>::DEFAULT_SCORER;
    /// assert_eq!(None,     scorer("sa", &"New York",      "New York",      0));
    /// assert_eq!(Some(49), scorer("sa", &"Sacramento",    "Sacramento",    1));
    /// assert_eq!(Some(35), scorer("sa", &"Kansas",        "Kansas",        2));
    /// assert_eq!(Some(35), scorer("sa", &"Mesa",          "Mesa",          3));
    /// assert_eq!(None,     scorer("sa", &"Phoenix",       "Phoenix",       4));
    /// assert_eq!(None,     scorer("sa", &"Philadelphia",  "Philadelphia",  5));
    /// assert_eq!(Some(49), scorer("sa", &"San Antonio",   "San Antonio",   6));
    /// assert_eq!(Some(49), scorer("sa", &"San Diego",     "San Diego",     7));
    /// assert_eq!(None,     scorer("sa", &"Dallas",        "Dallas",        8));
    /// assert_eq!(Some(49), scorer("sa", &"San Francisco", "San Francisco", 9));
    /// assert_eq!(None,     scorer("sa", &"Austin",        "Austin",        10));
    /// assert_eq!(None,     scorer("sa", &"Jacksonville",  "Jacksonville",  11));
    /// assert_eq!(Some(49), scorer("sa", &"San Jose",      "San Jose",      12));
    /// ```
    #[cfg(feature = "fuzzy")]
    pub const DEFAULT_SCORER: Scorer<'a, T> =
        &|input, _option, string_value, _idx| -> Option<i64> {
            DEFAULT_MATCHER.fuzzy_match(string_value, input)
        };

    #[cfg(not(feature = "fuzzy"))]
    pub const DEFAULT_SCORER: Scorer<'a, T> =
        &|input, _option, string_value, _idx| -> Option<i64> {
            let filter = input.to_lowercase();
            match string_value.to_lowercase().contains(&filter) {
                true => Some(0),
                false => None,
            }
        };

    /// Default page size.
    pub const DEFAULT_PAGE_SIZE: usize = crate::config::DEFAULT_PAGE_SIZE;

    /// Default value of vim mode.
    pub const DEFAULT_VIM_MODE: bool = crate::config::DEFAULT_VIM_MODE;

    /// Default starting cursor index.
    pub const DEFAULT_STARTING_CURSOR: usize = 0;

    /// Default cursor behaviour on filter input change.
    /// Defaults to true.
    pub const DEFAULT_RESET_CURSOR: bool = true;

    /// Default filter input enabled behaviour.
    /// Defaults to true.
    pub const DEFAULT_FILTER_INPUT_ENABLED: bool = true;

    /// Default help message.
    pub const DEFAULT_HELP_MESSAGE: Option<&'a str> =
        Some("↑↓ to move, enter to select, type to filter");

    /// Creates a [Select] with the provided message and options, along with default configuration values.
    pub fn new(message: &'a str, options: Vec<T>) -> Self {
        Self {
            message,
            options,
            help_message: Self::DEFAULT_HELP_MESSAGE,
            page_size: Self::DEFAULT_PAGE_SIZE,
            vim_mode: Self::DEFAULT_VIM_MODE,
            starting_cursor: Self::DEFAULT_STARTING_CURSOR,
            reset_cursor: Self::DEFAULT_RESET_CURSOR,
            filter_input_enabled: Self::DEFAULT_FILTER_INPUT_ENABLED,
            scorer: Self::DEFAULT_SCORER,
            formatter: Self::DEFAULT_FORMATTER,
            render_config: get_configuration(),
            starting_filter_input: None,
        }
    }

    /// Sets the help message of the prompt.
    pub fn with_help_message(mut self, message: &'a str) -> Self {
        self.help_message = Some(message);
        self
    }

    /// Removes the set help message.
    pub fn without_help_message(mut self) -> Self {
        self.help_message = None;
        self
    }

    /// Sets the page size.
    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    /// Enables or disables vim_mode.
    pub fn with_vim_mode(mut self, vim_mode: bool) -> Self {
        self.vim_mode = vim_mode;
        self
    }

    /// Sets the scoring function.
    pub fn with_scorer(mut self, scorer: Scorer<'a, T>) -> Self {
        self.scorer = scorer;
        self
    }

    /// Sets the formatter.
    pub fn with_formatter(mut self, formatter: OptionFormatter<'a, T>) -> Self {
        self.formatter = formatter;
        self
    }

    /// Sets the starting cursor index.
    ///
    /// This index might be overridden if the `reset_cursor` option is set to true (default)
    /// and starting_filter_input is set to something other than None.
    pub fn with_starting_cursor(mut self, starting_cursor: usize) -> Self {
        self.starting_cursor = starting_cursor;
        self
    }

    /// Sets the starting filter input
    pub fn with_starting_filter_input(mut self, starting_filter_input: &'a str) -> Self {
        self.starting_filter_input = Some(starting_filter_input);
        self
    }

    /// Sets the reset_cursor behaviour. Defaults to true.
    ///
    /// When there's an input change that results in a different list of options being displayed,
    /// whether by filtering or re-ordering, the cursor will be reset to highlight the first option.
    pub fn with_reset_cursor(mut self, reset_cursor: bool) -> Self {
        self.reset_cursor = reset_cursor;
        self
    }

    /// Disables the filter input, which means the user will not be able to filter the options
    /// by typing.
    ///
    /// This is useful when you want to simplify the UX if the filter does not add any value,
    /// such as when the list is already short.
    pub fn without_filtering(mut self) -> Self {
        self.filter_input_enabled = false;
        self
    }

    /// Sets the provided color theme to this prompt.
    ///
    /// Note: The default render config considers if the NO_COLOR environment variable
    /// is set to decide whether to render the colored config or the empty one.
    ///
    /// When overriding the config in a prompt, NO_COLOR is no longer considered and your
    /// config is treated as the only source of truth. If you want to customize colors
    /// and still support NO_COLOR, you will have to do this on your end.
    pub fn with_render_config(mut self, render_config: RenderConfig<'a>) -> Self {
        self.render_config = render_config;
        self
    }

    /// Parses the provided behavioral and rendering options and prompts
    /// the CLI user for input according to the defined rules.
    ///
    /// Returns the owned object selected by the user.
    pub fn prompt(self) -> InquireResult<T> {
        self.raw_prompt().map(|op| op.value)
    }

    /// Parses the provided behavioral and rendering options and prompts
    /// the CLI user for input according to the defined rules.
    ///
    /// This method is intended for flows where the user skipping/cancelling
    /// the prompt - by pressing ESC - is considered normal behavior. In this case,
    /// it does not return `Err(InquireError::OperationCanceled)`, but `Ok(None)`.
    ///
    /// Meanwhile, if the user does submit an answer, the method wraps the return
    /// type with `Some`.
    pub fn prompt_skippable(self) -> InquireResult<Option<T>> {
        match self.prompt() {
            Ok(answer) => Ok(Some(answer)),
            Err(InquireError::OperationCanceled) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Parses the provided behavioral and rendering options and prompts
    /// the CLI user for input according to the defined rules.
    ///
    /// Returns a [`ListOption`](crate::list_option::ListOption) containing
    /// the index of the selection and the owned object selected by the user.
    ///
    /// This method is intended for flows where the user skipping/cancelling
    /// the prompt - by pressing ESC - is considered normal behavior. In this case,
    /// it does not return `Err(InquireError::OperationCanceled)`, but `Ok(None)`.
    ///
    /// Meanwhile, if the user does submit an answer, the method wraps the return
    /// type with `Some`.
    pub fn raw_prompt_skippable(self) -> InquireResult<Option<ListOption<T>>> {
        match self.raw_prompt() {
            Ok(answer) => Ok(Some(answer)),
            Err(InquireError::OperationCanceled) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Parses the provided behavioral and rendering options and prompts
    /// the CLI user for input according to the defined rules.
    ///
    /// Returns a [`ListOption`](crate::list_option::ListOption) containing
    /// the index of the selection and the owned object selected by the user.
    pub fn raw_prompt(self) -> InquireResult<ListOption<T>> {
        let (input_reader, terminal) = get_default_terminal()?;
        let mut backend = Backend::new(input_reader, terminal, self.render_config)?;
        self.prompt_with_backend(&mut backend)
    }

    pub(crate) fn prompt_with_backend<B: SelectBackend>(
        self,
        backend: &mut B,
    ) -> InquireResult<ListOption<T>> {
        SelectPrompt::new(self)?.prompt(backend)
    }
}
