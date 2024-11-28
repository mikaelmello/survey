mod action;
mod config;
mod prompt;
#[cfg(test)]
#[cfg(feature = "crossterm")]
mod test;

pub use action::*;

use crate::{
    config::get_configuration,
    error::{InquireError, InquireResult},
    formatter::StringFormatter,
    prompts::prompt::Prompt,
    terminal::get_default_terminal,
    ui::{Backend, PasswordBackend, RenderConfig},
    validator::StringValidator,
};

use self::prompt::PasswordPrompt;

/// Display modes of the text input of a password prompt.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PasswordDisplayMode {
    /// Password text input is not rendered at all, no indication of input.
    Hidden,

    /// Characters of the password text input are rendered marked as different
    /// characters, such as asterisks. These characters are configured in the
    /// render config.
    Masked,

    /// Same as [Masked](PasswordDisplayMode::Masked), but the last typed
    /// character remains unmasked immediately after typing it.
    UnmaskedLastChar,

    /// Password text input is fully rendered as a normal input, just like
    /// [Text](crate::Text) prompts.
    Full,
}

/// Prompt meant for secretive text inputs.
///
/// By default, the password prompt behaves like a standard one you'd see in common CLI applications: the user has no UI indicators about the state of the current input. They do not know how many characters they typed, or which character they typed, with no option to display the current text input.
///
/// However, you can still customize these and other behaviors if you wish:
/// - **Standard display mode**: Set the display mode of the text input among hidden, masked and full via the `PasswordDisplayMode` enum.
///   - Hidden: default behavior, no UI indicators.
///   - Masked: behaves like a normal text input, except that all characters of the input are masked to a special character, which is `'*'` by default but can be customized via `RenderConfig`.
///   - Full: behaves like a normal text input, no modifications.
/// - **Toggle display mode**: When enabling this feature by calling the `with_display_toggle_enabled()` method, you allow the user to toggle between the standard display mode set and the full display mode.
///   - If you have set the standard display mode to hidden (which is also the default) or masked, the user can press `Ctrl+R` to change the display mode to `Full`, and `Ctrl+R` again to change it back to the standard one.
///   - Obviously, if you have set the standard display mode to `Full`, pressing `Ctrl+R` won't cause any changes.
/// - **Confirmation**: By default, the password will have a confirmation flow where the user will be asked for the input twice and the two responses will be compared. If they differ, an error message is shown and the user is prompted again.
///   - By default, a "Confirmation:" message is shown for the confirmation prompts, but this can be modified by setting a custom confirmation message only shown the second time, using the `with_custom_confirmation_message()` method.
///   - If confirmation is not desired, it can be turned off using the `without_confirmation()` method.
/// - **Help message**: Message displayed at the line below the prompt.
/// - **Formatter**: Custom formatter in case you need to pre-process the user input before showing it as the final answer.
///   - By default, it prints eight asterisk characters: `********`.
/// - **Validators**: Custom validators to make sure a given submitted input pass the specified requirements, e.g. not allowing empty inputs or requiring special characters.
///   - No validators are on by default.
///
/// Remember that for CLI applications it is standard to not allow use any display modes other than `Hidden` and to not allow the user to see the text input in any way. _Use the customization options at your discretion_.
///
/// # Example
///
/// ```no_run
///  use inquire::{validator::{StringValidator, Validation}, Password, PasswordDisplayMode};
///
///  let validator = |input: &str| if input.chars().count() < 10 {
///      Ok(Validation::Invalid("Keys must have at least 10 characters.".into()))
///  } else {
///      Ok(Validation::Valid)
///  };
///
///  let name = Password::new("Encryption Key:")
///      .with_display_toggle_enabled()
///      .with_display_mode(PasswordDisplayMode::Hidden)
///      .with_custom_confirmation_message("Encryption Key (confirm):")
///      .with_custom_confirmation_error_message("The keys don't match.")
///      .with_validator(validator)
///      .with_formatter(&|_| String::from("Input received"))
///      .with_help_message("It is recommended to generate a new one only for this purpose")
///      .prompt();
///
///  match name {
///      Ok(_) => println!("This doesn't look like a key."),
///      Err(_) => println!("An error happened when asking for your key, try again later."),
///  }
/// ```
#[derive(Clone)]
pub struct Password<'a> {
    /// Message to be presented to the user.
    pub message: &'a str,

    /// Message to be presented to the user when confirming the input.
    pub custom_confirmation_message: Option<&'a str>,

    /// Error to be presented to the user when password confirmation fails.
    pub custom_confirmation_error_message: Option<&'a str>,

    /// Help message to be presented to the user.
    pub help_message: Option<&'a str>,

    /// Function that formats the user input and presents it to the user as the final rendering of the prompt.
    pub formatter: StringFormatter<'a>,

    /// How the password input is displayed to the user.
    pub display_mode: PasswordDisplayMode,

    /// Whether to allow the user to toggle the display of the current password input by pressing the Ctrl+R hotkey.
    pub enable_display_toggle: bool,

    /// Whether to ask for input twice to see if the provided passwords are the same.
    pub enable_confirmation: bool,

    /// Collection of validators to apply to the user input.
    ///
    /// Validators are executed in the order they are stored, stopping at and displaying to the user
    /// only the first validation error that might appear.
    ///
    /// The possible error is displayed to the user one line above the prompt.
    pub validators: Vec<Box<dyn StringValidator>>,

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

impl<'a> Password<'a> {
    /// Default formatter, set to always display `"********"` regardless of input length.
    pub const DEFAULT_FORMATTER: StringFormatter<'a> = &|_| String::from("********");

    /// Default validators added to the [Password] prompt, none.
    pub const DEFAULT_VALIDATORS: Vec<Box<dyn StringValidator>> = vec![];

    /// Default help message.
    pub const DEFAULT_HELP_MESSAGE: Option<&'a str> = None;

    /// Default value for the allow display toggle variable.
    pub const DEFAULT_ENABLE_DISPLAY_TOGGLE: bool = false;

    /// Default value for the enable confirmation variable.
    pub const DEFAULT_ENABLE_CONFIRMATION: bool = true;

    /// Default password display mode.
    pub const DEFAULT_DISPLAY_MODE: PasswordDisplayMode = PasswordDisplayMode::Hidden;

    /// Creates a [Password] with the provided message and default options.
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            custom_confirmation_message: None,
            custom_confirmation_error_message: None,
            enable_confirmation: Self::DEFAULT_ENABLE_CONFIRMATION,
            enable_display_toggle: Self::DEFAULT_ENABLE_DISPLAY_TOGGLE,
            display_mode: Self::DEFAULT_DISPLAY_MODE,
            help_message: Self::DEFAULT_HELP_MESSAGE,
            formatter: Self::DEFAULT_FORMATTER,
            validators: Self::DEFAULT_VALIDATORS,
            render_config: get_configuration(),
        }
    }

    /// Sets the help message of the prompt.
    pub fn with_help_message(mut self, message: &'a str) -> Self {
        self.help_message = Some(message);
        self
    }

    /// Sets the flag to enable display toggling.
    pub fn with_display_toggle_enabled(mut self) -> Self {
        self.enable_display_toggle = true;
        self
    }

    /// Disables the confirmation step of the prompt.
    pub fn without_confirmation(mut self) -> Self {
        self.enable_confirmation = false;
        self
    }

    /// Sets the prompt message when asking for the password confirmation.
    pub fn with_custom_confirmation_message(mut self, message: &'a str) -> Self {
        self.custom_confirmation_message.replace(message);
        self
    }

    /// Sets the prompt error message when password confirmation fails.
    pub fn with_custom_confirmation_error_message(mut self, message: &'a str) -> Self {
        self.custom_confirmation_error_message.replace(message);
        self
    }

    /// Sets the standard display mode for the prompt.
    pub fn with_display_mode(mut self, mode: PasswordDisplayMode) -> Self {
        self.display_mode = mode;
        self
    }

    /// Sets the formatter.
    pub fn with_formatter(mut self, formatter: StringFormatter<'a>) -> Self {
        self.formatter = formatter;
        self
    }

    /// Adds a validator to the collection of validators. You might want to use this feature
    /// in case you need to limit the user to specific choices, such as requiring
    /// special characters in the password.
    ///
    /// Validators are executed in the order they are stored, stopping at and displaying to the user
    /// only the first validation error that might appear.
    ///
    /// The possible error is displayed to the user one line above the prompt.
    pub fn with_validator<V>(mut self, validator: V) -> Self
    where
        V: StringValidator + 'static,
    {
        self.validators.push(Box::new(validator));
        self
    }

    /// Adds the validators to the collection of validators in the order they are given.
    ///
    /// Validators are executed in the order they are stored, stopping at and displaying to the user
    /// only the first validation error that might appear.
    ///
    /// The possible error is displayed to the user one line above the prompt.
    pub fn with_validators(mut self, validators: &[Box<dyn StringValidator>]) -> Self {
        for validator in validators {
            #[allow(suspicious_double_ref_op)]
            self.validators.push(validator.clone());
        }
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
    /// This method is intended for flows where the user skipping/cancelling
    /// the prompt - by pressing ESC - is considered normal behavior. In this case,
    /// it does not return `Err(InquireError::OperationCanceled)`, but `Ok(None)`.
    ///
    /// Meanwhile, if the user does submit an answer, the method wraps the return
    /// type with `Some`.
    pub fn prompt_skippable(self) -> InquireResult<Option<String>> {
        match self.prompt() {
            Ok(answer) => Ok(Some(answer)),
            Err(InquireError::OperationCanceled) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Parses the provided behavioral and rendering options and prompts
    /// the CLI user for input according to the defined rules.
    pub fn prompt(self) -> InquireResult<String> {
        let (input_reader, terminal) = get_default_terminal()?;
        let mut backend = Backend::new(input_reader, terminal, self.render_config)?;
        self.prompt_with_backend(&mut backend)
    }

    pub(crate) fn prompt_with_backend<B: PasswordBackend>(
        self,
        backend: &mut B,
    ) -> InquireResult<String> {
        PasswordPrompt::from(self).prompt(backend)
    }
}
