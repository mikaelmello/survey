use crate::{
    error::InquireResult,
    formatter::StringFormatter,
    input::Input,
    prompts::prompt::{ActionResult, Prompt},
    ui::PasswordBackend,
    validator::{ErrorMessage, StringValidator, Validation},
    InquireError, Password, PasswordDisplayMode,
};

use super::{action::PasswordPromptAction, config::PasswordConfig};

// Helper type for representing the password confirmation flow.
struct PasswordConfirmation<'a> {
    // The message of the prompt.
    pub message: &'a str,

    // The error message of the prompt.
    pub error_message: &'a str,

    // The input to confirm.
    pub input: Input,
}

pub struct PasswordPrompt<'a> {
    message: &'a str,
    config: PasswordConfig,
    default: Option<&'a str>,
    help_message: Option<&'a str>,
    input: Input,
    current_mode: PasswordDisplayMode,
    confirmation: Option<PasswordConfirmation<'a>>, // if `None`, confirmation is disabled, `Some(_)` confirmation is enabled
    confirmation_stage: bool,
    formatter: StringFormatter<'a>,
    validators: Vec<Box<dyn StringValidator>>,
    error: Option<ErrorMessage>,
}

impl<'a> From<Password<'a>> for PasswordPrompt<'a> {
    fn from(so: Password<'a>) -> Self {
        let confirmation = match so.enable_confirmation {
            true => Some(PasswordConfirmation {
                message: so.custom_confirmation_message.unwrap_or("Confirmation:"),
                error_message: so
                    .custom_confirmation_error_message
                    .unwrap_or("The answers don't match."),
                input: Input::new(),
            }),
            false => None,
        };

        Self {
            message: so.message,
            config: (&so).into(),
            default: so.default,
            help_message: so.help_message,
            current_mode: so.display_mode,
            confirmation,
            confirmation_stage: false,
            formatter: so.formatter,
            validators: so.validators,
            input: Input::new(),
            error: None,
        }
    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(val: &'a str) -> Self {
        Password::new(val)
    }
}

impl<'a> PasswordPrompt<'a> {
    fn active_input_mut(&mut self) -> &mut Input {
        if let Some(c) = &mut self.confirmation {
            if self.confirmation_stage {
                return &mut c.input;
            }
        }

        &mut self.input
    }

    fn toggle_display_mode(&mut self) -> ActionResult {
        let new_mode = match self.current_mode {
            PasswordDisplayMode::Hidden | PasswordDisplayMode::Masked => PasswordDisplayMode::Full,
            PasswordDisplayMode::Full => self.config.display_mode,
        };

        if new_mode != self.current_mode {
            self.current_mode = new_mode;
            ActionResult::NeedsRedraw
        } else {
            ActionResult::Clean
        }
    }

    fn confirmation_step(&mut self) -> ConfirmationStepResult {
        let cur_answer = self.cur_answer().to_owned();
        match &mut self.confirmation {
            None => ConfirmationStepResult::NoConfirmationRequired,
            Some(confirmation) => {
                if self.confirmation_stage {
                    if cur_answer == confirmation.input.content() {
                        ConfirmationStepResult::ConfirmationValidated
                    } else {
                        self.confirmation_stage = false;
                        confirmation.input.clear();
                        ConfirmationStepResult::ConfirmationInvalidated(ErrorMessage::Custom(
                            confirmation.error_message.to_owned(),
                        ))
                    }
                } else {
                    confirmation.input.clear();
                    self.confirmation_stage = true;

                    ConfirmationStepResult::ConfirmationPending
                }
            }
        }
    }

    fn validate_current_answer(&self) -> InquireResult<Validation> {
        for validator in &self.validators {
            match validator.validate(self.cur_answer()) {
                Ok(Validation::Valid) => {}
                Ok(Validation::Invalid(msg)) => return Ok(Validation::Invalid(msg)),
                Err(err) => return Err(InquireError::Custom(err)),
            }
        }

        Ok(Validation::Valid)
    }

    fn cur_answer(&self) -> &str {
        self.input.content()
    }
}

impl<'a, Backend> Prompt<Backend> for PasswordPrompt<'a>
where
    Backend: PasswordBackend,
{
    type Config = PasswordConfig;
    type InnerAction = PasswordPromptAction;
    type Output = String;

    fn message(&self) -> &str {
        self.message
    }

    fn config(&self) -> &PasswordConfig {
        &self.config
    }

    fn format_answer(&self, answer: &String) -> String {
        (self.formatter)(answer)
    }

    fn setup(&mut self) -> InquireResult<()> {
        if let Some(val) = self.default {
            self.message = val;
        }

        Ok(())
    }

    fn pre_cancel(&mut self) -> InquireResult<bool> {
        if let Some(confirmation) = &mut self.confirmation {
            if self.confirmation_stage {
                confirmation.input.clear();
                self.confirmation_stage = false;
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn submit(&mut self) -> InquireResult<Option<String>> {
        if let Validation::Invalid(msg) = self.validate_current_answer()? {
            self.error = Some(msg);
            if self.config.display_mode == PasswordDisplayMode::Hidden {
                self.input.clear();
            }
            return Ok(None);
        }

        let confirmation = self.confirmation_step();

        let cur_answer = self.cur_answer().to_owned();

        let result = match confirmation {
            ConfirmationStepResult::NoConfirmationRequired
            | ConfirmationStepResult::ConfirmationValidated => Some(cur_answer),
            ConfirmationStepResult::ConfirmationPending => None,
            ConfirmationStepResult::ConfirmationInvalidated(message) => {
                self.error = Some(message);
                self.input.clear();
                None
            }
        };

        Ok(result)
    }

    fn handle(&mut self, action: PasswordPromptAction) -> InquireResult<ActionResult> {
        let result = match action {
            PasswordPromptAction::ValueInput(input_action) => {
                self.active_input_mut().handle(input_action).into()
            }
            PasswordPromptAction::ToggleDisplayMode => self.toggle_display_mode(),
        };

        Ok(result)
    }

    fn render(&self, backend: &mut Backend) -> InquireResult<()> {
        if let Some(err) = &self.error {
            backend.render_error_message(err)?;
        }

        match self.current_mode {
            PasswordDisplayMode::Hidden => {
                backend.render_prompt(self.message)?;

                match &self.confirmation {
                    Some(confirmation) if self.confirmation_stage => {
                        backend.render_prompt(confirmation.message)?;
                    }
                    _ => {}
                }
            }
            PasswordDisplayMode::Masked => {
                backend.render_prompt_with_masked_input(self.message, &self.input)?;

                match &self.confirmation {
                    Some(confirmation) if self.confirmation_stage => {
                        backend.render_prompt_with_masked_input(
                            confirmation.message,
                            &confirmation.input,
                        )?;
                    }
                    _ => {}
                }
            }
            PasswordDisplayMode::Full => {
                backend.render_prompt_with_full_input(self.message, &self.input)?;

                match &self.confirmation {
                    Some(confirmation) if self.confirmation_stage => {
                        backend.render_prompt_with_full_input(
                            confirmation.message,
                            &confirmation.input,
                        )?;
                    }
                    _ => {}
                }
            }
        }

        if let Some(message) = self.help_message {
            backend.render_help_message(message)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmationStepResult {
    NoConfirmationRequired,
    ConfirmationPending,
    ConfirmationValidated,
    ConfirmationInvalidated(ErrorMessage),
}
