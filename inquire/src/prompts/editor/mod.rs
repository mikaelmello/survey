mod action;
mod config;
mod prompt;

pub use action::*;

use std::{
    env,
    ffi::{OsStr, OsString},
};

use once_cell::sync::Lazy;

use crate::{
    error::{InquireError, InquireResult},
    formatter::StringFormatter,
    prompts::prompt::Prompt,
    terminal::get_default_terminal,
    ui::{Backend, EditorBackend, RenderConfig},
    validator::StringValidator,
};

use self::prompt::EditorPrompt;

static DEFAULT_EDITOR: Lazy<OsString> = Lazy::new(get_default_editor_command);

/// This prompt is meant for cases where you need the user to write some text that might not fit in a single line, such as long descriptions or commit messages.
///
/// This prompt is gated via the `editor` because it depends on the `tempfile` crate.
///
/// This prompt's behavior is to ask the user to either open the editor - by pressing the `e` key - or submit the current text - by pressing the `enter` key. The user can freely open and close the editor as they wish, until they either cancel or submit.
///
/// The editor opened is set by default to `nano` on Unix environments and `notepad` on Windows environments. Additionally, if there's an editor set in either the `EDITOR` or `VISUAL` environment variables, it is used instead.
///
/// If the user presses `esc` while the editor is not open, it will be interpreted as the user canceling (or skipping) the operation, in which case the prompt call will return `Err(InquireError::OperationCanceled)`.
///
/// If the user presses `enter` without ever modyfing the temporary file, it will be treated as an empty submission. If this is unwanted behavior, you can control the user input by using validators.
///
/// Finally, this prompt allows a great range of customizable options as all others:
///
/// - **Prompt message**: Main message when prompting the user for input, `"What is your name?"` in the example above.
/// - **Help message**: Message displayed at the line below the prompt.
/// - **Editor command and its args**: If you want to override the selected editor, you can pass over the command and additional args.
/// - **File extension**: Custom extension for the temporary file, useful as a proxy for proper syntax highlighting for example.
/// - **Predefined text**: Pre-defined text to be written to the temporary file before the user is allowed to edit it.
/// - **Validators**: Custom validators to the user's input, displaying an error message if the input does not pass the requirements.
/// - **Formatter**: Custom formatter in case you need to pre-process the user input before showing it as the final answer.
///   - By default, a successfully submitted answer is displayed to the user simply as `<received>`.
#[derive(Clone)]
pub struct Editor<'a> {
    /// Message to be presented to the user.
    pub message: &'a str,

    /// Command to open the editor.
    pub editor_command: &'a OsStr,

    /// Args to pass to the editor.
    pub editor_command_args: &'a [&'a OsStr],

    /// Extension of the file opened in the text editor, useful for syntax highlighting.
    ///
    /// The dot prefix should be included in the string, e.g. ".rs".
    pub file_extension: &'a str,

    /// Predefined text to be present on the text file on the text editor.
    pub predefined_text: Option<&'a str>,

    /// Help message to be presented to the user.
    pub help_message: Option<&'a str>,

    /// Function that formats the user input and presents it to the user as the final rendering of the prompt.
    pub formatter: StringFormatter<'a>,

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

impl<'a> Editor<'a> {
    /// Default formatter, set to [DEFAULT_STRING_FORMATTER](crate::formatter::DEFAULT_STRING_FORMATTER)
    pub const DEFAULT_FORMATTER: StringFormatter<'a> = &|_| String::from("<received>");

    /// Default validators added to the [Editor] prompt, none.
    pub const DEFAULT_VALIDATORS: Vec<Box<dyn StringValidator>> = vec![];

    /// Default help message.
    pub const DEFAULT_HELP_MESSAGE: Option<&'a str> = None;

    /// Creates a [Editor] with the provided message and default options.
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            editor_command: &DEFAULT_EDITOR,
            editor_command_args: &[],
            file_extension: ".txt",
            predefined_text: None,
            help_message: Self::DEFAULT_HELP_MESSAGE,
            validators: Self::DEFAULT_VALIDATORS,
            formatter: Self::DEFAULT_FORMATTER,
            render_config: RenderConfig::default(),
        }
    }

    /// Sets the help message of the prompt.
    pub fn with_help_message(mut self, message: &'a str) -> Self {
        self.help_message = Some(message);
        self
    }

    /// Sets the predefined text to be written into the temporary file.
    pub fn with_predefined_text(mut self, text: &'a str) -> Self {
        self.predefined_text = Some(text);
        self
    }

    /// Sets the file extension of the temporary file.
    pub fn with_file_extension(mut self, file_extension: &'a str) -> Self {
        self.file_extension = file_extension;
        self
    }

    /// Sets the command to open the editor.
    pub fn with_editor_command(mut self, editor_command: &'a OsStr) -> Self {
        self.editor_command = editor_command;
        self
    }

    /// Sets the args for the command to open the editor.
    pub fn with_args(mut self, args: &'a [&'a OsStr]) -> Self {
        self.editor_command_args = args;
        self
    }

    /// Sets the formatter.
    pub fn with_formatter(mut self, formatter: StringFormatter<'a>) -> Self {
        self.formatter = formatter;
        self
    }

    /// Adds a validator to the collection of validators. You might want to use this feature
    /// in case you need to require certain features from the user's answer, such as
    /// defining a limit of characters.
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
    /// You might want to use this feature in case you need to require certain features
    /// from the user's answer, such as defining a limit of characters.
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

    pub fn prompt_immediate(self) -> InquireResult<String> {
        let (input_reader, terminal) = get_default_terminal()?;
        let mut backend = Backend::new(input_reader, terminal, self.render_config)?;
        EditorPrompt::new(self)?.prompt_immediate(&mut backend)
    }

    pub(crate) fn prompt_with_backend<B: EditorBackend>(
        self,
        backend: &mut B,
    ) -> InquireResult<String> {
        EditorPrompt::new(self)?.prompt(backend)
    }
}

fn get_default_editor_command() -> OsString {
    let mut default_editor = if cfg!(windows) {
        String::from("notepad")
    } else {
        String::from("nano")
    };

    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            default_editor = editor;
        }
    }

    if let Ok(editor) = env::var("VISUAL") {
        if !editor.is_empty() {
            default_editor = editor;
        }
    }

    default_editor.into()
}
