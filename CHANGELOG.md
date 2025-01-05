# Changelog

<!-- next-header -->

## [Unreleased] <!-- ReleaseDate -->

- Don't require static lifetime for autocompleter and validator.
- Fix autocomplete suggestions not being updated after a suggestion is accepted. Thanks @moritz-hoelting and @istudyatuni for reporting and fixing it!
- Fix incorrect cursor placement when inputting CJK characters. Thanks @phostann (#270) for reporting it!
- Removed unused dependency (newline-converter). Thanks @jonassmedegaard (#267) for catching it!

## [0.7.5] - 2024-04-23

- Fix user-provided ANSI escape codes from being removed when rendering.
  - Introduced on 0.7.0, this regression was making it impossible to have colorised text inside the prompt.
  - Now ANSI escape codes are properly emitted when rendering the prompt in the terminal.

## [0.7.4] - 2024-03-25

- Fix unexpected behaviors of `keep_filter` option in MultiSelect prompts:
  - Filter input is now correcly getting reset **only when** `keep_filter == false`.
  - When the filter input is reset, the list of options is now correctly reset as well. Thanks @Swivelgames for reporting [#238](https://github.com/mikaelmello/inquire/issues/238).

## [0.7.3] - 2024-03-21

- Fix cursor occasionally blinking in unexpected places.

## [0.7.2] - 2024-03-17

- Pressing Ctrl+D now cancels the prompt. Thanks @mikecvet for the PR!
- Add support for `h` and `l` bindings when vim_mode is enabled on MultiSelect prompts, clearing or selecting all options respectively. Thanks @afh for the PR!
- Fix render issue [#233](https://github.com/mikaelmello/inquire/issues/233) where cursor positioning at the end of a prompt was incorrect. Thanks @msrd0 and @Sydonian for reporting!

## [0.7.1] - 2024-03-10

- Fix render issue [#228](https://github.com/mikaelmello/inquire/pull/228) when using `console` crate as the terminal backend. Thanks @maospr for reporting.

## [0.7.0] - 2024-02-24

### Breaking Changes

- The Select and Multiselect Filter now scores input and is now expected to return an `Option<i64>`, making it possible to order/rank the list of options. [#176](https://github.com/mikaelmello/inquire/pull/176)
  `None`: Will not be displayed in the list of options.
  `Some(score)`: score determines the order of options, higher score, higher on the list of options.
- Improved user experience on Password prompts. When there is a validation error, the input is cleared if the password is rendered using the `Hidden` display mode, matching the user expectation of having to write the password from scratch again. Thanks to @CM-IV for the questions on #149!
- Allow lifetime customization of RenderConfig. [#101](https://github.com/mikaelmello/inquire/pull/101). Thanks to @arturfast for the suggestion [#95](https://github.com/mikaelmello/inquire/issues/95).
- Implement fuzzy search as default on Select and MultiSelect prompts. [#176](https://github.com/mikaelmello/inquire/pull/176)
- Revamped keybindings for DateSelect.

### Features

- Add one-liner helpers for quick scripts. [#144](https://github.com/mikaelmello/inquire/pull/144).
- Add new option on MultiSelect prompts to set all options to be selected by default. Thanks to @conikeec for the suggestion (#151)!
- Add new option on Select/MultiSelect prompts allowing to reset selection to the first item on filter-input changes. [#176](https://github.com/mikaelmello/inquire/pull/176)
- Emacs-like keybindings added where applicable:
  - Ctrl-p/Ctrl-n for up/down
  - Ctrl-b/Ctrl-f for left/right
  - Ctrl-j/Ctrl-g for enter/cancel
- Vim keybindings are always supported in DateSelect prompts.
- Added 'with_starting_filter_input' to both Select and MultiSelect, which allows for setting an initial value to the filter section of the prompt.
- Added starting_input for CustomType. [#194](https://github.com/mikaelmello/inquire/pull/194)
- Added 'without_filtering' to both Select and MultiSelect, useful when you want to simplify the UX if the filter does not add any value, such as when the list is already short.
- Added 'with_answered_prompt_prefix' to RenderConfig to allow customization of answered prompt prefix.
- Improved rendering, with optimizations on incremental rendering and terminal resizing.

### Fixes

- Fixed typos in the code's comments.
- Fixed issue where inquire, using termion, would crash when receiving piped inputs.

### Dependency changes (some breaking)

- Upgraded underlying `termion` crate from v1.5 to v2.0.
- Upgraded underlying `bitflags` from v1 to v2, which affects the `Attributes` and `KeyModifiers` crates. If you use any of bitflag's methods directly, you might be affected, refer to the [bitflags changelog](https://github.com/bitflags/bitflags/releases/tag/2.0.0) for more information.
- Removed `thiserror` dependency in favor of implementing `InquireError` by hand. [#146](https://github.com/mikaelmello/inquire/issues/146)
- Raised MSRV to 1.66 due to requirements in downstream dependencies.
- MSRV is now explicitly set in the package definition.
- Replaced `lazy_static` with `once_cell` as `once_cell::sync::Lazy` is being standardized and `lazy_static` is not actively maintained anymore.
- Added `fuzzy-matcher` as an optional dependency for fuzzy filtering in Select and MultiSelect prompts [#176](https://github.com/mikaelmello/inquire/pull/176)

## [0.6.2] - 2023-05-07

- Allow usage of ANSI escape codes in prompts. [#136](https://github.com/mikaelmello/inquire/pull/136). Thanks to [@JimLynchCodes](https://github.com/JimLynchCodes) for reporting on [#135](https://github.com/mikaelmello/inquire/issues/135).

## [0.6.1] - 2023-04-08

- Fix incorrect highlighting of lists when filtered. [#110](https://github.com/mikaelmello/inquire/pull/110). Thanks to [@prime31](https://github.com/prime31) for reporting on [#106](https://github.com/mikaelmello/inquire/issues/106).

## [0.6.0] - 2023-03-03

### Breaking Changes

- Selected option can now be styled independently of other options through `RenderConfig::with_selected_option()`.
- Now selected options are highlighted cyan by default.
- Output dialogs on `stderr` instead of `stdout` [#89](https://github.com/mikaelmello/inquire/pull/89).
- New Minimum Supported Rust Version: 1.58.1.

## [0.5.3] - 2023-01-09

- Addition of `with_starting_date(NaiveDate)` to `DateSelect` prompts.
  - Equivalent to `with_default(NaiveDate)`, but with a more descriptive name.

## [0.5.2] - 2022-11-01

- Fixed typo in the default error message when a password confirmation does not match. Thanks to @woodruffw for the PR! [#79](https://github.com/mikaelmello/inquire/pull/79)
  - Releases containing the typo: v0.5.0 and v0.5.1.

## [0.5.1] - 2022-10-31

- Removed use of `bool::then_some` feature to keep minimum supported Rust version on 1.56.0.

## [0.5.0] - 2022-10-31

### Breaking Changes

**`Password` prompts now enable a secondary confirmation prompt by default:**

- Added support for password confirmation, which can be oupted-out of by adding the `without_confirmation()` method into the `Password` builder chain. Thanks to @hampuslidin for the PR! [#73](https://github.com/mikaelmello/inquire/pull/73)

## [0.4.0] - 2022-09-27

### Breaking Changes

**Multiple changes to the `CustomType` prompt:**

- Added support for validators, separating concerns between parsing and validating parsed values.
- Decoupled default value formatting from the default value property. Now you can set default values without a specific formatter to accompany them.
- Input is not cleared anymore when the parsing or validation fails.

**New autocompletion mechanism for `Text` prompts**

- Existing methods still work, you just have to update `with_suggester` calls to `with_autocomplete`.
- To know more about the new possibilities, check the updated documentation on the repository's README.

### Other changes

- Added shorthand method `rgb(r: u8, g: u8, b: u8)` to create a `Color` struct from RGB components. Thanks to @tpoliaw for the PR! [#73](https://github.com/mikaelmello/inquire/pull/73)

## [0.3.0] - 2022-08-19

### Breaking Changes

Features #1 to #4 are all breaking changes and could break the compilation of your program.

Fix #2 represents a change in usability and might be an unexpected behavior.

### Features

#### 1. Completer

`Completer` for `Text` prompts, allowing users to auto-update their text input by pressing `tab` and not having to navigate through a suggestion list.

It takes the current input and return an optional suggestion. If any, the prompter will replace the current input with the received suggestion. `Completer` is an alias for `&'a dyn Fn(&str) -> Result<Option<String>, CustomUserError>`.

_The auto-completion API will be revamped for v0.4.0, watch [#69](https://github.com/mikaelmello/inquire/pull/69)._

---

#### 2. Support for custom prompt prefix in finished prompts.

Added `answered_prompt_prefix` configuration on `RenderConfig`, allowing users to set custom prefixes (e.g. a check mark) to prompts that have already been answered.

Additionally, prompts that have been answered are now differed by a `>` prefix instead of the usual `?`.

Cheers to @href for the suggestion! [#44](https://github.com/mikaelmello/inquire/pull/44)

---

#### 3. User-provided operations can be fallible.

Input validation, suggestions and completions are now fallible operations.

The return type of validators has been changed to `Result<Validation, CustomUserError>`. This means that validating the input can now be a fallible operation. The docs contain more thorough explanations and full-featured examples.

- Successful executions of the validator should return a variant of the `Validation` enum, which can be either `Valid` or `Invalid(ErrorMessage)`.
- Unsuccessful executions return a `CustomUserError` type, which is an alias for `Box<dyn std::error::Error + Send + Sync + 'static>`.

The return type of suggesters has also been changed to allow fallible executions. The return type in successful executions continues to be `Vec<String>`, while `CustomUserError` is used with errors.

---

#### 4. Validators are traits instead of closures.

All builtin validators have been turned into traits, with structs instead of macros as implementations.

This change makes it easier to share the validators throughout the code, especially if these carry their own owned data. For example, consider a validator that uses a compiled regular expression to verify the input. That validator can now be built as a new-type struct that encapsulates the regex.

Closures can still be used as before, but may not require to pass the argument type explicitly. The previous macros are now simply shorthands for the constructors of builtin validators.

### Fixes

- Fix a broken link in the `struct.Text` documentation.
- Suggestions are now always loaded at the start of a `Text` prompt.
  - Previously, suggestions were only loaded and displayed if the `Text` prompt was created with a pre-existing input value or after the user entered any input.
  - Now, even if the prompt is started without any input and the user hasn't done anything, suggestions are displayed.

### Changes

- Update `crossterm` and `console` to their latest versions.

## [0.2.1] - 2021-10-01

### Features

- Add `initial_value` property to `Text` prompts, which sets an initial value for the prompt's text input. Huge thanks to [@irevoire](https://github.com/irevoire) for the suggestion **and** implementation. [#34](https://github.com/mikaelmello/inquire/pull/34).

### Internals

- Multiple changes to fix general warnings appearing throughout the code.

## [0.2.0] - 2021-09-14

### Features

- Add `inquire::set_global_render_config` method to set a global RenderConfig object to be used as the default one for all prompts created after the call.
- Add [KEY_BINDINGS.md](KEY_BINDINGS.md) to register all key bindings registered by `inquire` prompts.

### Breaking changes

- `RenderConfig` was made `Copy`-able and prompts now contain a `RenderConfig` field where it previously held a `&'a RenderConfig`. Consequently, `with_render_config()` methods now accept a `RenderConfig` argument instead of `&'a RenderConfig`.

## [0.1.0] - 2021-09-14

No changes in this version.

This is a bump to v0.1.0 as per @jam1garner's advice on the Virtual RustConf Discord server.

The library is already featureful enough to warrant a higher version number, bumping us to a minor release while we are still on our path to stabilization.

## [0.0.11] - 2021-09-06

### Features

- Add [`Editor`](https://docs.rs/inquire/0.0.11/inquire/prompts/editor/struct.Editor.html) prompt.
- Add support to use `console` or `termion` as the library to handle terminals while keeping `crossterm` as the default choice.
- Canceling the prompt by pressing `ESC` is now a different behavior than interrupting the prompt by pressing `Ctrl+C`.
  - If the prompt is canceled, the final prompt render indicates to the user that it was canceled via a `<canceled>` text, which is customizable via RenderConfig, and the prompt method returns `Err(InquireError::OperationCanceled)`.
  - If the prompt is interrupted, the only clean-up action done is restoring the cursor position, and the prompt method returns `Err(InquireError::OperationInterrupted)`.
- Add a `prompt_skippable` method for all prompts.
  - This method is intended for flows where the user skipping/cancelling the prompt - by pressing ESC - is considered normal behavior. In this case, it does not return `Err(InquireError::OperationCanceled)`, but `Ok(None)`. Meanwhile, if the user does submit an answer, the method wraps the return type with `Some`.

### Improvements

- Removed need to add `use inquire::validator::InquireLength` when using one of the length-related built-in validators.
- Cursor should not ficker anymore in wrong positions on ~~Windows~~ slower terminals.
- Documentation on the `Color` enum used for render configuration has been improved.

### Fixes

- Fix dependencies the crate had on macros provided by the `builtin_validators` feature, making it now compile when the feature is not turned on.

## [0.0.10] - 2021-08-29

### Features

- Use native terminal cursors in text inputs by default.
- Use native terminal cursor on date prompts when an optional style sheet for the selected cursor token was defined as `None`. The default behavior is still a custom style sheet which highlights the two columns pertaining to a date, instead of using a native cursor which can only highlight one column.
- Respect NO_COLOR environment variable when prompt uses the default render configuration.

### Fixes

- By using a new method to identify the length of the rendered prompt, we avoid possible rendering errors (edge cases) when a string can not render into a single line in the terminal due to a smaller width. Inner calculations could previously predict that the rendered string would fit, by considering that 1 grapheme = 1 column width, but this is not true for e.g. emojis. Now we use unicode_width to fix this behavior.
- Fixed case where Select/MultiSelect prompts were panicking when a user pressed the down arrow on an empty list, which happens when a filter input does not match any options. #30
- Fixed incorrect indexes on the output of MultiSelect prompts, where the indexes inside a `ListOption` struct were relative to the output instead of the original input vector. #31
- Fixed case where IO errors due to not finding a tty devices were not being caught and transformed to `InquireError::NotTTY`. #28

## [0.0.9] - 2021-08-28

### General

- Improve docs on the differences between `raw_prompt` and `prompt` on Select and MultiSelect prompts.
- Bump version of `crossterm` dependency

### Fixes

- Regression introduced on v0.0.8 where select prompts were panicking when user pressed enter while no options were displayed (due to filter input). Tracked by #29 and tests were added for this to not happen again.

## [0.0.8] - 2021-08-25

### Features

- **Password display toggle**: By enabling this option in `Password` prompts via `with_display_toggle_enabled()`, the application user can choose to display the current text input for the password by pressing `Ctrl+R`, and hide it back by pressing the hotkey again. #18
- **Render mask of password input**: `Password` prompts now support another render mode of the text input. Before, the only behavior was to not render anything about the current password input. Now, if the developer so chooses, they can activate the `Masked` mode where the current text input will be masked with special characters such as `'*'`. #19
- **PageUp, PageDown, Home and End hotkeys**: PageUp and PageDown are now supported in `Select`, `MultiSelect` and `Text` (suggestions list) prompts, where they go a page up or down in the current list. Also, for `Select` and `MultiSelect` prompts, the Home and End keys were mapped to go to the start or end of the list, respectively. #17
- **Indication that list is scrollable**: Now option lists, present in `Select`, `MultiSelect` and `Text` prompts, indicate whether there are more options other than the ones currently displayed. Little arrows are displayed at the top or bottom of the list indicating to which positions the user can scroll. #8
- **Generic option types for Select and MultiSelect prompts**: Now, `Select` and `MultiSelect` prompts accept any type of options as input, allowing developers to pass a vector of owned objects and get back the full object picked by the user. #9

### Fixes

- **Handling of new-line characters in user-provided strings**: When putting `\n` on strings such as prompt messages, the library previously did not render it very well and did not account for it when cleaning the current prompt. This is fixed and you are free to create multi-line prompts! #15
- **Lines larger than terminal width broke rendering**: When lines that were larger than the terminal width were rendered, it caused the internal line counter (used to clean the prompt) to be off, leading to buggy behavior. This was fixed by retrieving the terminal size at the start of the prompt. #21

## [0.0.7] - 2021-08-20

### Features

- Add possibility to set custom rendering config, allowing users to set:
  - Custom colors
  - Custom prefixes for several prompts
  - Custom checkboxes
- Add "placeholder" feature for prompts with text input

## [0.0.6] - 2021-07-26

- Add [previously non-existing] documentation.
- Add [CustomType](https://github.com/mikaelmello/inquire#customtype) prompt
- Add revamped auto-completion support for Text prompts

## [0.0.5] - 2021-07-19

- All function arguments now accept closures by having their type changed to `&dyn Fn`.
- Improved input UX
  - Cursor added for better editing experience
  - Features/shortcuts added: Ctrl+Left, Ctrl+Right, Home, End, Delete, Ctrl+Delete

## [0.0.4] - 2021-07-14

- Add a custom error enum `InquireError`, improving error handling for library users.
- Improve support for validator functions, allowing the use of closures.
- Change the terminal back-end from termion to crossterm, adding Windows support for this library.

## [0.0.3] - 2021-07-07

- Reduce package footprint
- Add custom parser option to Confirm prompt
- Add DateSelect prompt

## [History]

- Wasn't documented :)

<!-- next-url -->

[unreleased]: https://github.com/mikaelmello/inquire/compare/v0.7.5...HEAD
[0.7.5]: https://github.com/mikaelmello/inquire/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/mikaelmello/inquire/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/mikaelmello/inquire/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/mikaelmello/inquire/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/mikaelmello/inquire/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/mikaelmello/inquire/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/mikaelmello/inquire/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/mikaelmello/inquire/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/mikaelmello/inquire/compare/v0.5.3...v0.6.0
[0.5.3]: https://github.com/mikaelmello/inquire/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/mikaelmello/inquire/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/mikaelmello/inquire/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/mikaelmello/inquire/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/mikaelmello/inquire/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/mikaelmello/inquire/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/mikaelmello/inquire/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/mikaelmello/inquire/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mikaelmello/inquire/compare/v0.0.11...v0.1.0
[0.0.11]: https://github.com/mikaelmello/inquire/compare/v0.0.10...v0.0.11
[0.0.10]: https://github.com/mikaelmello/inquire/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/mikaelmello/inquire/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/mikaelmello/inquire/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/mikaelmello/inquire/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/mikaelmello/inquire/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/mikaelmello/inquire/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/mikaelmello/inquire/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/mikaelmello/inquire/compare/v0.0.2...v0.0.3
[history]: https://github.com/mikaelmello/inquire/compare/11e6f3b961477fbc19adc3c5322ff159c1f606f5...v0.0.2
