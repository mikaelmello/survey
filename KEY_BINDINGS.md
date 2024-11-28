# Key Bindings

This file lists all of the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
| -------------------------------- | ----------------------- |
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Canceling and interrupting a prompt have two different meanings. Canceling is defined specially for when the end user is allowed to skip a prompt, the library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches the `CanceledOperation` error transforming it into a `Ok(None)` result. Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as termination commands.

## Text Input

These key bindings may be used with all prompts that ask the user for text input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`]. The [`Editor`] prompt is not included because it opens a separate text editor for text input.


| **command**                         | **description**                                 |
| ----------------------------------- | ----------------------------------------------- |
| <kbd>character</kbd>                | Insert the character into the input.            |
| <kbd>left</kbd>                     | Move the cursor back one character.             |
| <kbd>right</kbd>                    | Move the cursor forward one character.          |
| <kbd>ctrl</kbd> + <kbd>left</kbd>   | Move one word to the left of the cursor.        |
| <kbd>ctrl</kbd> + <kbd>right</kbd>  | Move one word to the right of the cursor.       |
| <kbd>home</kbd>                     | Move cursor to the start of the line*.          |
| <kbd>end</kbd>                      | Move cursor to the end of the line*.            |
| <kbd>backspace</kbd>                | Delete one character to the left of the cursor. |
| <kbd>delete</kbd>                   | Delete the character at the cursor.             |
| <kbd>ctrl</kbd> + <kbd>delete</kbd> | Delete one word to the right of the cursor.     |

\* Key bindings not supported on [`Select`] and [`MultiSelect`] prompts.

## Text Prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the current current text input.                        |
| <kbd>up</kbd>        | When suggestions are displayed, move cursor one row up.       |
| <kbd>down</kbd>      | When suggestions are displayed, move cursor one row down.     |
| <kbd>page up</kbd>   | When suggestions are displayed, move cursor one page up.      |
| <kbd>page down</kbd> | When suggestions are displayed, move cursor one page down.    |
| <kbd>tab</kbd>       | Replace current input with the resulting suggestion if any.   |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Select Prompts

These key bindings may be used in [`Select`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the current highlighted option.                        |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## MultiSelect Prompts

These key bindings may be used in [`MultiSelect`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the options currently selected.                        |
| <kbd>space</kbd>     | Toggle the selection of the current highlighted option.       |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| <kbd>left</kbd>      | Unselect all options.                                         |
| <kbd>right</kbd>     | Select all options.                                           |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## DateSelect Prompts

These key bindings may be used in the interactive calendar of the [`DateSelect`] prompt.


| **command**                              | **description**                                               |
| ---------------------------------------- | ------------------------------------------------------------- |
| <kbd>space bar</kbd> or <kbd>enter</kbd> | Submit the current highlighted date.                          |
| <kbd>up</kbd>                            | Move cursor one row up.                                       |
| <kbd>down</kbd>                          | Move cursor one row down.                                     |
| <kbd>left</kbd>                          | Move cursor one column to the left.                           |
| <kbd>right</kbd>                         | Move cursor one column to the right.                          |
| <kbd>k</kbd>                             | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>                             | Move cursor one row down when vim mode is enabled.            |
| <kbd>h</kbd>                             | Move cursor one column to the left when vim mode is enabled.  |
| <kbd>l</kbd>                             | Move cursor one column to the right when vim mode is enabled. |
| <kbd>ctrl</kbd> + <kbd>up</kbd>          | Move calendar back by one year.                               |
| <kbd>ctrl</kbd> + <kbd>down</kbd>        | Move calendar forward by one year.                            |
| <kbd>ctrl</kbd> + <kbd>left</kbd>        | Move calendar back by one month.                              |
| <kbd>ctrl</kbd> + <kbd>right</kbd>       | Move calendar forward by one month.                           |

## Editor Prompts

These key bindings may be used in [`Editor`] prompts.

| **command**      | **description**                                                |
| ---------------- | -------------------------------------------------------------- |
| <kbd>e</kbd>     | Open the editor.                                               |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |


## Password Prompts

These key bindings may be used in [`Password`] prompts.

| **command**                    | **description**                                                                        |
|--------------------------------|----------------------------------------------------------------------------------------|
| <kbd>ctrl</kbd> + <kbd>r</kbd> | Toggles between [`Masked`]/[`UnmaskedLastChar`] and [`Full`] (unmasked) display modes. |


[`Text`]: https://docs.rs/inquire/*/inquire/prompts/text/struct.Text.html
[`DateSelect`]: https://docs.rs/inquire/*/inquire/prompts/dateselect/struct.DateSelect.html
[`Select`]: https://docs.rs/inquire/*/inquire/prompts/select/struct.Select.html
[`MultiSelect`]: https://docs.rs/inquire/*/inquire/prompts/multiselect/struct.MultiSelect.html
[`Confirm`]: https://docs.rs/inquire/*/inquire/prompts/confirm/struct.Confirm.html
[`Editor`]: https://docs.rs/inquire/*/inquire/prompts/editor/struct.Editor.html
[`customtype`]: https://docs.rs/inquire/*/inquire/struct.CustomType.html
[`Password`]: https://docs.rs/inquire/*/inquire/prompts/password/struct.Password.html
[`Masked`]: https://docs.rs/inquire/latest/inquire/enum.PasswordDisplayMode.html#variant.Masked
[`UnmaskedLastChar`]: https://docs.rs/inquire/latest/inquire/enum.PasswordDisplayMode.html#variant.UnmaskedLastChar
[`Full`]: https://docs.rs/inquire/latest/inquire/enum.PasswordDisplayMode.html#variant.Full
