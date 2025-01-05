use crate::{error::InquireResult, Confirm, CustomType, Password, Text};

/// This function is a helpful one-liner to prompt the user for the confirmation of an action.
///
/// Under the hood, it is equivalent to calling `inquire::Confirm::new(message).prompt()`.
/// See the documentation for [`Confirm`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for confirmation.
///
/// # Returns
///
/// * `InquireResult<bool>`: An enum that represents the result of the prompt operation. If the operation is successful,
///   it returns `InquireResult::Ok(bool)` where the bool represents the user's answer to the confirmation prompt.
///   `true` is for "yes" and `false` is for "no". If the operation encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// match prompt_confirmation("Are you sure you want to continue?") {
///     InquireResult::Ok(true) => println!("User confirmed."),
///     InquireResult::Ok(false) => println!("User did not confirm."),
///     InquireResult::Err(err) => println!("An error occurred: {}", err),
/// }
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_confirmation<M>(message: M) -> InquireResult<bool>
where
    M: AsRef<str>,
{
    Confirm::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a text input.
///
/// Under the hood, it is equivalent to calling `inquire::Text::new(message).prompt()`.
/// See the documentation for [`Text`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<String>`: An enum that represents the result of the prompt operation. If the operation is successful,
///   it returns `InquireResult::Ok(String)` where the String represents the user's input. If the operation encounters an
///   error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let name = prompt_text("What is your name?")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_text<M>(message: M) -> InquireResult<String>
where
    M: AsRef<str>,
{
    Text::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a password, or any secret text.
///
/// Under the hood, it is equivalent to calling `inquire::Password::new(message).prompt()`.
/// See the documentation for [`Password`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<String>`: An enum that represents the result of the prompt operation. If the operation is successful,
///   it returns `InquireResult::Ok(String)` where the String represents the user's input. If the operation encounters an
///   error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let token = prompt_secret("Access Token:")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_secret<M>(message: M) -> InquireResult<String>
where
    M: AsRef<str>,
{
    Password::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a date.
///
/// Under the hood, it is equivalent to calling `inquire::DateSelect::new(message).prompt()`.
/// See the documentation for [`crate::DateSelect`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<NaiveDate>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(NaiveDate)` where NaiveDate's value is the date selected by the user. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let token = prompt_secret("Access Token:")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
#[cfg(feature = "date")]
pub fn prompt_date<M>(message: M) -> InquireResult<chrono::NaiveDate>
where
    M: AsRef<str>,
{
    crate::DateSelect::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a number and parse it to f64.
///
/// Under the hood, it is equivalent to calling `inquire::CustomType::<f64>::new(message).prompt()`.
/// See the documentation for [`CustomType`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<f64>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(f64)` where f64 is the number parsed from the user's input. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let kilograms = prompt_f64("Weight (kg):")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_f64<M>(message: M) -> InquireResult<f64>
where
    M: AsRef<str>,
{
    CustomType::<f64>::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a number and parse it to f32.
///
/// Under the hood, it is equivalent to calling `inquire::CustomType::<f32>::new(message).prompt()`.
/// See the documentation for [`CustomType`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<f32>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(f32)` where f32 is the number parsed from the user's input. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let kilograms = prompt_f32("Weight (kg):")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_f32<M>(message: M) -> InquireResult<f32>
where
    M: AsRef<str>,
{
    CustomType::<f32>::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a number and parse it to u64.
///
/// Under the hood, it is equivalent to calling `inquire::CustomType::<u64>::new(message).prompt()`.
/// See the documentation for [`CustomType`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<u64>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(u64)` where u64 is the number parsed from the user's input. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let kilograms = prompt_u64("Weight (kg):")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_u64<M>(message: M) -> InquireResult<u64>
where
    M: AsRef<str>,
{
    CustomType::<u64>::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a number and parse it to u32.
///
/// Under the hood, it is equivalent to calling `inquire::CustomType::<u32>::new(message).prompt()`.
/// See the documentation for [`CustomType`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<u32>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(u32)` where u32 is the number parsed from the user's input. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let kilograms = prompt_u32("Weight (kg):")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_u32<M>(message: M) -> InquireResult<u32>
where
    M: AsRef<str>,
{
    CustomType::<u32>::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a number and parse it to usize.
///
/// Under the hood, it is equivalent to calling `inquire::CustomType::<usize>::new(message).prompt()`.
/// See the documentation for [`CustomType`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<usize>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(usize)` where usize is the number parsed from the user's input. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let kilograms = prompt_usize("Weight (kg):")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_usize<M>(message: M) -> InquireResult<usize>
where
    M: AsRef<str>,
{
    CustomType::<usize>::new(message.as_ref()).prompt()
}

/// This function is a helpful one-liner to prompt the user for a number and parse it to u128.
///
/// Under the hood, it is equivalent to calling `inquire::CustomType::<u128>::new(message).prompt()`.
/// See the documentation for [`CustomType`] for more information on its behavior.
///
/// # Arguments
///
/// * `message`: A message that implements the `AsRef<str>` trait. This message will be displayed to the user
///   when asking for input.
///
/// # Returns
///
/// * `InquireResult<u128>`: An enum that represents the result of the prompt operation. If the operation is successful,
///     it returns `InquireResult::Ok(u128)` where u128 is the number parsed from the user's input. If the operation
///     encounters an error, it returns `InquireResult::Err(InquireError)`.
///
/// # Example
///
/// ``` no_run
/// # use inquire::{*, error::*};
/// let kilograms = prompt_u128("Weight (kg):")?;
/// # inquire::error::InquireResult::Ok(())
/// ```
///
/// # Errors
///
/// This function will return an error if there is a problem interacting with the terminal, or if the user
/// cancels the operation by pressing `Ctrl+C`.
pub fn prompt_u128<M>(message: M) -> InquireResult<u128>
where
    M: AsRef<str>,
{
    CustomType::<u128>::new(message.as_ref()).prompt()
}
