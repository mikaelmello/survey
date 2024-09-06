//! Type aliases and default implementations for functions called as formatters
//! of a given input.
//!
//! Formatters receive the user input to a given prompt and return a formatted
//! output `String`, which is displayed to the user as the submitted value.
//!
//! # Example
//!
//! **Prompt code**
//!
//! ```no_run
//! use inquire::formatter::StringFormatter;
//! use inquire::Text;
//!
//! let formatter: StringFormatter = &|s| {
//!     let mut c = s.chars();
//!     match c.next() {
//!         None => String::from("No name given"),
//!         Some(f) => {
//!             String::from("My name is ")
//!                 + f.to_uppercase().collect::<String>().as_str()
//!                 + c.as_str()
//!         }
//!     }
//! };
//!
//! let name = Text::new("What's your name?")
//!     .with_formatter(formatter)
//!     .prompt();
//!
//! match name {
//!     Ok(_) => {}
//!     Err(err) => println!("Error: {}", err),
//! }
//! ```
//!
//! **Before submission (pressing Enter)**
//!
//! ```text
//! ? What's your name? mikael
//! ```
//!
//! **After submission**
//!
//! ```text
//! ? What's your name? My name is Mikael
//! ```

use crate::list_option::ListOption;

/// Type alias for formatters that receive a string slice as the input,
/// required by [Text](crate::Text) and [Password](crate::Password) for example.
///
/// Formatters receive the user input and return a [String] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use inquire::formatter::StringFormatter;
///
/// let formatter: StringFormatter = &|i| i.to_lowercase();
/// assert_eq!(String::from("times square"), formatter("Times Square"));
/// assert_eq!(String::from("times square"), formatter("times square"));
/// ```
pub type StringFormatter<'a> = &'a dyn Fn(&str) -> String;

/// Type alias for formatters used in [Confirm](crate::Confirm) prompts.
///
/// Formatters receive the user input and return a [String] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use inquire::formatter::BoolFormatter;
///
/// let formatter: BoolFormatter = &|i| match i {
///     true => String::from("si"),
///     false => String::from("no"),
/// };
/// assert_eq!(String::from("si"), formatter(true));
/// assert_eq!(String::from("no"), formatter(false));
/// ```
pub type BoolFormatter<'a> = &'a dyn Fn(bool) -> String;

/// Type alias for formatters used in [Select](crate::Select) prompts.
///
/// Formatters receive the user input and return a [String] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use inquire::list_option::ListOption;
/// use inquire::formatter::OptionFormatter;
///
/// let formatter: OptionFormatter<str> = &|i| format!("Option {}: '{}'", i.index + 1, i.value);
/// assert_eq!(String::from("Option 1: 'a'"), formatter(ListOption::new(0, "a")));
/// assert_eq!(String::from("Option 2: 'b'"), formatter(ListOption::new(1, "b")));
/// ```
pub type OptionFormatter<'a, T> = &'a dyn Fn(ListOption<&T>) -> String;

/// Type alias for formatters used in [`MultiSelect`](crate::MultiSelect) prompts.
///
/// Formatters receive the user input and return a [String] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use inquire::list_option::ListOption;
/// use inquire::formatter::MultiOptionFormatter;
///
/// let formatter: MultiOptionFormatter<str> = &|opts| {
///     let len = opts.len();
///     let options = match len {
///         1 => "option",
///         _ => "options",
///     };
///     format!("You selected {} {}", len, options)
/// };
///
/// let mut ans = vec![ListOption::new(0, "a")];
/// assert_eq!(String::from("You selected 1 option"), formatter(&ans));
///
/// ans.push(ListOption::new(3, "d"));
/// assert_eq!(String::from("You selected 2 options"), formatter(&ans));
/// ```
pub type MultiOptionFormatter<'a, T> = &'a dyn Fn(&[ListOption<&T>]) -> String;

use crate::list_option::CountedListOption;
/// Type alias for formatters used in [`MultiCount`](crate::MultiCount) prompts.
///
/// Formatters receive the user input and return a [(u32, String)] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use inquire::list_option::{ListOption, CountedListOption};
/// use inquire::formatter::MultiCountFormatter;
///
/// let formatter: MultiCountFormatter<str> = &|opts| {
///     let len = opts.len();
///     let options = match len {
///         1 => "option",
///         _ => "options",
///     };
///     let values = match len {
///         1 => "value",
///         _ => "values",
///     };
///     let counts = match len {
///        1 => "count",
///        _ => "counts",
///     };
///     format!("You selected {} {} with {}: {:?} and {}: {:?}", len, options, values, opts.iter().map(|c| c.list_option.value).collect::<Vec<_>>(), counts, opts.iter().map(|c| c.count).collect::<Vec<_>>())
/// };
///
/// let mut ans = vec![CountedListOption::new(1, ListOption::new(1,"a"))];
/// assert_eq!(String::from("You selected 1 option with value: [\"a\"] and count: [1]"), formatter(&ans));
///
/// ans.push(CountedListOption::new(3, ListOption::new(3, "d")));
/// assert_eq!(String::from("You selected 2 options with values: [\"a\", \"d\"] and counts: [1, 3]"), formatter(&ans));
/// ```
pub type MultiCountFormatter<'a, T> = &'a dyn Fn(&[CountedListOption<&T>]) -> String;

/// Type alias for formatters used in [`CustomType`](crate::CustomType) prompts.
///
/// Formatters receive the user input and return a [String] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use inquire::CustomType;
/// use inquire::formatter::CustomTypeFormatter;
///
/// let formatter: CustomTypeFormatter<f64> = &|i| format!("${:.2}", i);
///
/// assert_eq!(String::from("$12.33"), formatter(12.33));
/// assert_eq!(String::from("$44.91"), formatter(44.9123));
/// assert_eq!(String::from("$45.00"), formatter(44.998));
/// ```
pub type CustomTypeFormatter<'a, T> = &'a dyn Fn(T) -> String;

#[cfg(feature = "date")]

/// Type alias for formatters used in [`DateSelect`](crate::DateSelect) prompts.
///
/// Formatters receive the user input and return a [String] to be displayed
/// to the user as the final answer.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use inquire::formatter::DateFormatter;
///
/// let formatter: DateFormatter = &|val| val.format("%d/%m/%Y").to_string();
///
/// assert_eq!(
///     String::from("25/07/2021"),
///     formatter(NaiveDate::from_ymd(2021, 7, 25)),
/// );
/// ```
pub type DateFormatter<'a> = &'a dyn Fn(chrono::NaiveDate) -> String;

/// String formatter used by default in inputs that return a `String` as input.
/// Its behavior is to just echo the received input.
///
/// # Examples
///
/// ```
/// use inquire::formatter::DEFAULT_STRING_FORMATTER;
///
/// let formatter = DEFAULT_STRING_FORMATTER;
/// assert_eq!(String::from("Times Square"), formatter("Times Square"));
/// assert_eq!(String::from("times sQuare"), formatter("times sQuare"));
/// ```
pub const DEFAULT_STRING_FORMATTER: StringFormatter<'_> = &|val| String::from(val);

/// String formatter used by default in [Confirm](crate::Confirm) prompts.
/// Translates `bool` to `"Yes"` and `false` to `"No"`.
///
/// # Examples
///
/// ```
/// use inquire::formatter::DEFAULT_BOOL_FORMATTER;
///
/// let formatter = DEFAULT_BOOL_FORMATTER;
/// assert_eq!(String::from("Yes"), formatter(true));
/// assert_eq!(String::from("No"), formatter(false));
/// ```
pub const DEFAULT_BOOL_FORMATTER: BoolFormatter<'_> = &|ans| {
    if ans {
        String::from("Yes")
    } else {
        String::from("No")
    }
};

#[cfg(feature = "date")]
/// String formatter used by default in [`DateSelect`](crate::DateSelect) prompts.
/// Prints the selected date in the format: Month Day, Year.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use inquire::formatter::DEFAULT_DATE_FORMATTER;
///
/// let formatter = DEFAULT_DATE_FORMATTER;
///
/// assert_eq!(
///     String::from("July 25, 2021"),
///     formatter(NaiveDate::from_ymd(2021, 7, 25)),
/// );
/// assert_eq!(
///     String::from("January 1, 2021"),
///     formatter(NaiveDate::from_ymd(2021, 1, 1)),
/// );
/// ```
pub const DEFAULT_DATE_FORMATTER: DateFormatter<'_> = &|val| val.format("%B %-e, %Y").to_string();
