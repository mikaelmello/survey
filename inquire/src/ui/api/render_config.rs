use std::env;

use super::{Color, StyleSheet, Styled};

/// Rendering configuration that can be applied to a prompt.
///
/// Render configurations can set mostly style sheets for particular
/// parts of the prompt layout. Additionally, it allows you to set
/// the content of a few tokens, such as prompt or error message prefixes.
///
/// # Example
///
/// ```
/// use inquire::ui::{Color, RenderConfig, Styled};
///
/// let empty: RenderConfig = RenderConfig::empty();
/// let default: RenderConfig = RenderConfig::default();
///
/// let prompt_prefix = Styled::new("$").with_fg(Color::DarkRed);
/// let mine = default.with_prompt_prefix(prompt_prefix);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct RenderConfig<'a> {
    /// Prefix added before prompts.
    ///
    /// Note: a space character will be added to separate the prefix
    /// and the prompt message.
    pub prompt_prefix: Styled<&'a str>,

    /// Prefix added before answered prompts.
    ///
    /// Note: a space character will be added to separate the prefix
    /// and the prompt message.
    pub answered_prompt_prefix: Styled<&'a str>,

    /// Style of the prompt message, applicable to all prompt types.
    pub prompt: StyleSheet,

    /// Render configuration of default values.
    ///
    /// Note: default values are displayed wrapped in parenthesis, e.g. (yes).
    /// Non-styled space characters is added before the default value display
    /// and after the default value, as separators.
    pub default_value: StyleSheet,

    /// Render configuration of placeholders.
    ///
    /// Note: placeholders are displayed wrapped in parenthesis, e.g. (yes).
    /// Non-styled space characters is added before the default value display
    /// and after the default value, as separators.
    pub placeholder: StyleSheet,

    /// Render configuration of help messages.
    ///
    /// Note: help messages are displayed wrapped in brackets, e.g. [Be careful!].
    pub help_message: StyleSheet,

    /// Character used to mask password text inputs when in mode
    /// [`Masked`](crate::prompts::PasswordDisplayMode::Masked) or
    /// [`UnmaskedLastChar`](crate::prompts::PasswordDisplayMode::UnmaskedLastChar).
    ///
    /// Note: Styles for masked text inputs are set in the
    /// [`text_input`](crate::ui::RenderConfig::text_input) configuration.
    pub password_mask: char,

    /// Style sheet for text inputs.
    ///
    /// Note: a non-styled space character is added before the text input as
    /// a separator from the prompt message (or default value display).
    pub text_input: StyleSheet,

    /// Render configuration of final prompt answers (submissions).
    ///
    /// Note: a non-styled space character is added before the answer as
    /// a separator from the prompt message (or default value display).
    pub answer: StyleSheet,

    /// Render configuration of the message printed in the place of an answer
    /// when the prompt is canceled by the user - by pressing ESC.
    ///
    /// Note: a non-styled space character is added before the indicator as
    /// a separator from the prompt message.
    pub canceled_prompt_indicator: Styled<&'a str>,

    /// Render configuration for error messages.
    pub error_message: ErrorMessageRenderConfig<'a>,

    /// Prefix for the current highlighted option.
    ///
    /// Note: a space character will be added to separate the prefix
    /// and the option value or the checkbox.
    pub highlighted_option_prefix: Styled<&'a str>,

    /// Prefix for the option listed at the top of the page, when it is possible
    /// to scroll up.
    ///
    /// Note: a space character will be added to separate the prefix
    /// and the option value or the checkbox.
    pub scroll_up_prefix: Styled<&'a str>,

    /// Prefix for the option listed at the bottom of the page, when it is possible
    /// to scroll down.
    ///
    /// Note: a space character will be added to separate the prefix
    /// and the option value or the checkbox.
    pub scroll_down_prefix: Styled<&'a str>,

    /// Selected checkbox in multi-select options.
    ///
    /// Note: a space character will be added to separate the checkbox
    /// from a possible prefix, and to separate the checkbox from the
    /// option value to the right.
    pub selected_checkbox: Styled<&'a str>,

    /// Unselected checkbox in multi-select options.
    ///
    /// Note: a space character will be added to separate the checkbox
    /// from a possible prefix, and to separate the checkbox from the
    /// option value to the right.
    pub unselected_checkbox: Styled<&'a str>,

    /// Definition of index prefixes in option lists.
    pub option_index_prefix: IndexPrefix,

    /// Style sheet for options.
    ///
    /// Note: a non-styled space character is added before the option value as
    /// a separator from the prefix.
    pub option: StyleSheet,

    /// Style sheet for the option that is currently selected. If the value is
    /// None, it will fall back to `option`.
    ///
    /// Note: a non-styled space character is added before the option value as
    /// a separator from the prefix.
    pub selected_option: Option<StyleSheet>,

    /// Render configuration for calendar

    #[cfg(feature = "date")]
    /// Render configuration for date prompts`
    pub calendar: calendar::CalendarRenderConfig<'a>,

    /// Style sheet of the hint in editor prompts.
    ///
    /// The hint is formatted as `[(e) to open {}, (enter) to submit]`
    /// with the editor name.
    #[cfg(feature = "editor")]
    pub editor_prompt: StyleSheet,
}

impl<'a> RenderConfig<'a> {
    /// RenderConfig in which no colors or attributes are applied.
    pub fn empty() -> Self {
        Self {
            prompt_prefix: Styled::new("?"),
            answered_prompt_prefix: Styled::new("?"),
            prompt: StyleSheet::empty(),
            default_value: StyleSheet::empty(),
            placeholder: StyleSheet::empty(),
            help_message: StyleSheet::empty(),
            text_input: StyleSheet::empty(),
            error_message: ErrorMessageRenderConfig::empty(),
            answer: StyleSheet::empty(),
            canceled_prompt_indicator: Styled::new("<canceled>"),
            password_mask: '*',
            highlighted_option_prefix: Styled::new(">"),
            scroll_up_prefix: Styled::new("^"),
            scroll_down_prefix: Styled::new("v"),
            selected_checkbox: Styled::new("[x]"),
            unselected_checkbox: Styled::new("[ ]"),
            option_index_prefix: IndexPrefix::None,
            option: StyleSheet::empty(),
            selected_option: None,

            #[cfg(feature = "date")]
            calendar: calendar::CalendarRenderConfig::empty(),

            #[cfg(feature = "editor")]
            editor_prompt: StyleSheet::empty(),
        }
    }

    /// RenderConfig where default colors and attributes are applied.
    pub fn default_colored() -> Self {
        Self {
            prompt_prefix: Styled::new("?").with_fg(Color::LightGreen),
            answered_prompt_prefix: Styled::new(">").with_fg(Color::LightGreen),
            prompt: StyleSheet::empty(),
            default_value: StyleSheet::empty(),
            placeholder: StyleSheet::new().with_fg(Color::DarkGrey),
            help_message: StyleSheet::empty().with_fg(Color::LightCyan),
            text_input: StyleSheet::empty(),
            error_message: ErrorMessageRenderConfig::default_colored(),
            password_mask: '*',
            answer: StyleSheet::empty().with_fg(Color::LightCyan),
            canceled_prompt_indicator: Styled::new("<canceled>").with_fg(Color::DarkRed),
            highlighted_option_prefix: Styled::new(">").with_fg(Color::LightCyan),
            scroll_up_prefix: Styled::new("^"),
            scroll_down_prefix: Styled::new("v"),
            selected_checkbox: Styled::new("[x]").with_fg(Color::LightGreen),
            unselected_checkbox: Styled::new("[ ]"),
            option_index_prefix: IndexPrefix::None,
            option: StyleSheet::empty(),
            selected_option: Some(StyleSheet::new().with_fg(Color::LightCyan)),

            #[cfg(feature = "date")]
            calendar: calendar::CalendarRenderConfig::default_colored(),

            #[cfg(feature = "editor")]
            editor_prompt: StyleSheet::new().with_fg(Color::DarkCyan),
        }
    }

    /// Sets the prompt prefix and its style sheet.
    pub fn with_prompt_prefix(mut self, prompt_prefix: Styled<&'a str>) -> Self {
        self.prompt_prefix = prompt_prefix;
        self
    }

    /// Sets the answered prompt prefix and its style sheet.
    pub fn with_answered_prompt_prefix(mut self, answered_prompt_prefix: Styled<&'a str>) -> Self {
        self.answered_prompt_prefix = answered_prompt_prefix;
        self
    }

    /// Sets style for text inputs.
    pub fn with_text_input(mut self, text_input: StyleSheet) -> Self {
        self.text_input = text_input;
        self
    }

    /// Sets the style sheet for default values.
    pub fn with_default_value(mut self, default_value: StyleSheet) -> Self {
        self.default_value = default_value;
        self
    }

    /// Sets the style sheet for help messages.
    pub fn with_help_message(mut self, help_message: StyleSheet) -> Self {
        self.help_message = help_message;
        self
    }

    /// Sets the style sheet for answers.
    pub fn with_answer(mut self, answer: StyleSheet) -> Self {
        self.answer = answer;
        self
    }

    /// Sets the render configuration for error messages.
    pub fn with_error_message(mut self, error_message: ErrorMessageRenderConfig<'a>) -> Self {
        self.error_message = error_message;
        self
    }

    /// Sets the styled component for prefixes in highlighted options.
    pub fn with_highlighted_option_prefix(
        mut self,
        highlighted_option_prefix: Styled<&'a str>,
    ) -> Self {
        self.highlighted_option_prefix = highlighted_option_prefix;
        self
    }

    /// Sets the styled component for prefixes in scroll-up indicators.
    pub fn with_scroll_up_prefix(mut self, scroll_up_prefix: Styled<&'a str>) -> Self {
        self.scroll_up_prefix = scroll_up_prefix;
        self
    }

    /// Sets the styled component for prefixes in scroll-down indicators.
    pub fn with_scroll_down_prefix(mut self, scroll_down_prefix: Styled<&'a str>) -> Self {
        self.scroll_down_prefix = scroll_down_prefix;
        self
    }

    /// Sets the styled component for selected checkboxes.
    pub fn with_selected_checkbox(mut self, selected_checkbox: Styled<&'a str>) -> Self {
        self.selected_checkbox = selected_checkbox;
        self
    }

    /// Sets the styled component for unselected checkboxes.
    pub fn with_unselected_checkbox(mut self, unselected_checkbox: Styled<&'a str>) -> Self {
        self.unselected_checkbox = unselected_checkbox;
        self
    }

    /// Sets the index prefix for option lists.
    pub fn with_option_index_prefix(mut self, index_prefix: IndexPrefix) -> Self {
        self.option_index_prefix = index_prefix;
        self
    }

    /// Sets the style sheet for option values.
    pub fn with_option(mut self, option: StyleSheet) -> Self {
        self.option = option;
        self
    }

    /// Sets the style sheet for currently selected option.
    pub fn with_selected_option(mut self, selected_option: Option<StyleSheet>) -> Self {
        self.selected_option = selected_option;
        self
    }

    /// Sets the indicator for canceled prompts.
    pub fn with_canceled_prompt_indicator(
        mut self,
        canceled_prompt_indicator: Styled<&'a str>,
    ) -> Self {
        self.canceled_prompt_indicator = canceled_prompt_indicator;
        self
    }

    #[cfg(feature = "date")]
    /// Sets the render configuration for calendars.
    pub fn with_calendar_config(mut self, calendar: calendar::CalendarRenderConfig<'a>) -> Self {
        self.calendar = calendar;
        self
    }

    #[cfg(feature = "editor")]
    /// Sets the render configuration for editor prompts.
    pub fn with_editor_prompt(mut self, editor_prompt: StyleSheet) -> Self {
        self.editor_prompt = editor_prompt;
        self
    }
}

impl<'a> Default for RenderConfig<'a> {
    fn default() -> Self {
        match env::var("NO_COLOR") {
            Ok(_) => Self::empty(),
            Err(_) => Self::default_colored(),
        }
    }
}

/// Definition of index prefixes in option lists.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndexPrefix {
    /// Lists of options will not display any hints regarding
    /// the position/index of the positions.
    None,

    /// A simple index (1-based) will be displayed before the
    /// option string representation.
    Simple,

    /// A simple index (1-based) will be displayed before the
    /// option string representation.
    ///
    /// The number representation of the index is padded with
    /// spaces so that the length is the same of the largest
    /// index. That is, if the list has 100 options, the first 9
    /// options will be rendered as `"  1", "  2", ...`. Then all
    /// indexes with two digits will be padded with one space, and
    /// finally the last option with index 100 will not need to be
    /// padded.
    SpacePadded,

    /// A simple index (1-based) will be displayed before the
    /// option string representation.
    ///
    /// The number representation of the index is padded with
    /// zeroes so that the length is the same of the largest
    /// index. That is, if the list has 100 options, the first 9
    /// options will be rendered as `"001", "002", ...`. Then all
    /// indexes with two digits will be padded with one zero, and
    /// finally the last option with index 100 will not need to be
    /// padded.
    ZeroPadded,
}

/// Render configuration for error messages.
#[derive(Copy, Clone, Debug)]
pub struct ErrorMessageRenderConfig<'a> {
    /// Prefix style.
    pub prefix: Styled<&'a str>,

    /// Separator style.
    ///
    /// Note: This separator is a space character. It might be useful to
    /// style it if you want to set a background color for error messages.
    pub separator: StyleSheet,

    /// Message style.
    pub message: StyleSheet,

    /// Default message used for validators that do not defined custom error messages.
    pub default_message: &'a str,
}

impl<'a> ErrorMessageRenderConfig<'a> {
    /// Render configuration in which no colors or attributes are applied.
    pub fn empty() -> Self {
        Self {
            prefix: Styled::new("#"),
            separator: StyleSheet::empty(),
            message: StyleSheet::empty(),
            default_message: "Invalid input.",
        }
    }

    /// Render configuration where default colors and attributes are applied.
    pub fn default_colored() -> Self {
        Self {
            prefix: Styled::new("#").with_fg(Color::LightRed),
            separator: StyleSheet::empty(),
            message: StyleSheet::empty().with_fg(Color::LightRed),
            default_message: "Invalid input.",
        }
    }

    /// Sets the prefix.
    pub fn with_prefix(mut self, prefix: Styled<&'a str>) -> Self {
        self.prefix = prefix;
        self
    }

    /// Sets the separator stylesheet.
    ///
    /// Note: This separator is a space character. It might be useful to
    /// style it if you want to set a background color for error messages.
    pub fn with_separator(mut self, separator: StyleSheet) -> Self {
        self.separator = separator;
        self
    }

    /// Sets the message stylesheet.
    pub fn with_message(mut self, message: StyleSheet) -> Self {
        self.message = message;
        self
    }
}

#[cfg(feature = "date")]
pub mod calendar {
    //! Module containing additional render config for date prompts.

    use super::{Color, StyleSheet, Styled};

    /// Calendar configuration for error messages.
    #[derive(Copy, Clone, Debug)]
    pub struct CalendarRenderConfig<'a> {
        /// Prefix style.
        pub prefix: Styled<&'a str>,

        /// Style sheet for the calendar header, e.g. january 2021.
        pub header: StyleSheet,

        /// Style sheet for the calendar week header, e.g. su mo tu we th fr sa.
        pub week_header: StyleSheet,

        /// Style sheet for the currently selected date.
        ///
        /// When `None`, no custom style sheet will be applied and the native
        /// terminal cursor will be used in the first char of the date number.
        ///
        /// When `Some(_)`, the style sheet will be applied to the two columns
        /// where the number is positioned, padded to spaces in the left if the
        /// number only has one digit. e.g. " 5" or "23".
        pub selected_date: Option<StyleSheet>,

        /// Style sheet for today's date, just for hinting purposes.
        pub today_date: StyleSheet,

        /// Style sheet for dates that are from the previous or next month
        /// displayed in the calendar.
        pub different_month_date: StyleSheet,

        /// Style sheet for dates that can not be selected due to the
        /// min/max settings.
        pub unavailable_date: StyleSheet,
    }

    impl<'a> CalendarRenderConfig<'a> {
        /// Render configuration in which no colors or attributes are applied.
        pub fn empty() -> Self {
            Self {
                prefix: Styled::new(">"),
                header: StyleSheet::empty(),
                week_header: StyleSheet::empty(),
                selected_date: None,
                today_date: StyleSheet::empty(),
                different_month_date: StyleSheet::empty(),
                unavailable_date: StyleSheet::empty(),
            }
        }

        /// Render configuration where default colors and attributes are applied.
        pub fn default_colored() -> Self {
            Self {
                prefix: Styled::new(">").with_fg(Color::LightGreen),
                header: StyleSheet::empty(),
                week_header: StyleSheet::empty(),
                selected_date: Some(
                    StyleSheet::empty()
                        .with_fg(Color::Black)
                        .with_bg(Color::Grey),
                ),
                today_date: StyleSheet::empty().with_fg(Color::LightGreen),
                different_month_date: StyleSheet::empty().with_fg(Color::DarkGrey),
                unavailable_date: StyleSheet::empty().with_fg(Color::DarkGrey),
            }
        }

        /// Sets the prefix.
        pub fn with_prefix(mut self, prefix: Styled<&'a str>) -> Self {
            self.prefix = prefix;
            self
        }
    }
}
