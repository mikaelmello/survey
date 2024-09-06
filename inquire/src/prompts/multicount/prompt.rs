use std::{cmp::Reverse, collections::BTreeSet, fmt::Display};

use crate::{
    error::InquireResult,
    formatter::MultiCountFormatter,
    input::{Input, InputActionResult},
    list_option::{CountedListOption, ListOption},
    prompts::prompt::{ActionResult, Prompt},
    type_aliases::Scorer,
    ui::MultiCountBackend,
    utils::paginate,
    validator::{ErrorMessage, MultiOptionValidator, Validation},
    InquireError, MultiCount,
};

use super::{action::MultiCountPromptAction, config::MultiCountConfig};

pub struct MultiCountPrompt<'a, T> {
    message: &'a str,
    config: MultiCountConfig,
    options: Vec<T>,
    string_options: Vec<String>,
    help_message: Option<&'a str>,
    cursor_index: usize,
    counts: BTreeSet<(usize, u32)>, // Likely records which ones are checked...?
    input: Option<Input>,
    scored_options: Vec<usize>,
    scorer: Scorer<'a, T>,
    formatter: MultiCountFormatter<'a, T>,
    validator: Option<Box<dyn MultiOptionValidator<T>>>,
    error: Option<ErrorMessage>,
}

impl<'a, T> MultiCountPrompt<'a, T>
where
    T: Display,
{
    pub fn new(mco: MultiCount<'a, T>) -> InquireResult<Self> {
        if mco.options.is_empty() {
            return Err(InquireError::InvalidConfiguration(
                "Must have at least one available option".into(),
            ));
        }

        // Check if the default is within bounds
        if let Some(default) = &mco.default {
            // i.e. if it has a default
            for (i, _) in default {
                if i >= &mco.options.len() {
                    return Err(InquireError::InvalidConfiguration(format!(
                        "Index {} is out-of-bounds for length {} of options",
                        i,
                        &mco.options.len()
                    )));
                }
            }
        }
        let string_options = mco.options.iter().map(T::to_string).collect(); // get the string representation of the options

        let scored_options = (0..mco.options.len()).collect(); // get the indices of the options
        let option_counts = mco
            .default
            .as_ref()
            .map(|d| {
                d.iter()
                    .cloned()
                    .filter(|(i, _)| *i < mco.options.len())
                    .collect()
            })
            .unwrap_or_default();

        let input = match mco.filter_input_enabled {
            true => Some(Input::new_with(
                mco.starting_filter_input.unwrap_or_default(),
            )),
            false => None,
        };

        Ok(Self {
            message: mco.message,
            config: (&mco).into(),
            options: mco.options,
            string_options,
            scored_options,
            help_message: mco.help_message,
            cursor_index: mco.starting_cursor,
            input,
            scorer: mco.scorer,
            formatter: mco.formatter,
            validator: mco.validator,
            error: None,
            counts: option_counts,
        })
    }

    fn move_cursor_up(&mut self, qty: usize, wrap: bool) -> ActionResult {
        let new_position = if wrap {
            let after_wrap = qty.saturating_sub(self.cursor_index);
            self.cursor_index
                .checked_sub(qty)
                .unwrap_or_else(|| self.scored_options.len().saturating_sub(after_wrap))
        } else {
            self.cursor_index.saturating_sub(qty)
        };

        self.update_cursor_position(new_position)
    }

    fn move_cursor_down(&mut self, qty: usize, wrap: bool) -> ActionResult {
        let mut new_position = self.cursor_index.saturating_add(qty);

        if new_position >= self.scored_options.len() {
            new_position = if self.scored_options.is_empty() {
                0
            } else if wrap {
                new_position % self.scored_options.len()
            } else {
                self.scored_options.len().saturating_sub(1)
            }
        }

        self.update_cursor_position(new_position)
    }

    fn update_cursor_position(&mut self, new_position: usize) -> ActionResult {
        if new_position != self.cursor_index {
            self.cursor_index = new_position;
            ActionResult::NeedsRedraw
        } else {
            ActionResult::Clean
        }
    }

    fn set_value(&mut self, qty: u32) -> ActionResult {
        // get the id of the currently selected option, if it exists
        let idx = match self.scored_options.get(self.cursor_index) {
            Some(idx) => idx,
            None => return ActionResult::Clean,
        };
        self.counts.insert((*idx, qty));
        ActionResult::NeedsRedraw
    }

    fn alter_value(&mut self, diff: i32) -> ActionResult {
        let idx = match self.scored_options.get(self.cursor_index) {
            Some(idx) => idx,
            None => return ActionResult::Clean,
        };

        let current_val = self.counts.iter().find(|(i, _)| i == idx);
        let old_val;
        let new_val;

        match current_val {
            // At the beginning, the tree is empty.
            // if the value is not in the tree, we insert it at a value of 1.
            None => {
                // because zero-vals are removed from the btree, we need to
                // jump-start with a basic value
                if diff > 0 {
                    self.counts.insert((*idx, diff as u32));
                }
            }
            // i.e. if it finds the pair
            Some((_, c)) => {
                old_val = *c;
                if diff < 0 {
                    new_val = (*c).saturating_sub(-diff as u32);
                } else {
                    new_val = (*c).saturating_add(diff as u32);
                }
                self.counts.insert((*idx, new_val));
                self.counts.remove(&(*idx, old_val));
            }
        }
        ActionResult::NeedsRedraw
    }

    fn clear_input_if_needed(&mut self, action: MultiCountPromptAction) -> ActionResult {
        if !self.config.keep_filter {
            return ActionResult::Clean;
        }

        match action {
            MultiCountPromptAction::SetCountCurrentOption(_)
            | MultiCountPromptAction::ClearSelections => {
                self.input.as_mut().map(Input::clear);
                ActionResult::NeedsRedraw
            }
            _ => ActionResult::Clean,
        }
    }

    // Used to validate the the current "answer" is valid.
    fn validate_current_answer(&self) -> InquireResult<Validation> {
        if let Some(validator) = &self.validator {
            // for each of the options, create a list option if the number is positive
            let mut option_counts = vec![];
            for (idx, count) in &self.counts {
                if *count > 0 {
                    let value = self.options.get(*idx).unwrap();
                    let lo = ListOption::new(*idx, value);
                    option_counts.push(lo);
                }
            }

            let res = validator.validate(&option_counts)?;
            Ok(res)
        } else {
            Ok(Validation::Valid)
        }
    }

    /// used to produce the actual vector of type values
    /// THIS IS WRONG AT THE MOMENT: NEED TO SEE WHAT IS GIONG ON WITH self.OPTIONS
    fn get_final_answer(&mut self) -> Vec<CountedListOption<T>> {
        let mut answer = vec![];

        // by iterating in descending order, we can safely
        // swap remove because the elements to the right
        // that we did not remove will not matter anymore.
        for pair in self.counts.iter().filter(|(_, val)| val > &0).rev() {
            let index = pair.0;
            let count = pair.1;
            let value = self.options.swap_remove(index);
            //let lo = (count, ListOption::new(index, value));
            let lo = CountedListOption {
                count,
                list_option: ListOption::new(index, value),
            };
            answer.push(lo);
        }
        answer.reverse();

        answer
    }

    /// This seems ok in terms of operation 18/03/2024 - no apparent dependence
    /// on the fact that it's counts not checks
    fn run_scorer(&mut self) {
        let content = match &self.input {
            Some(input) => input.content(),
            None => return,
        };
        // apply the scorer to the content and the options, map to a vec
        // of score and index
        let mut options = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| {
                (self.scorer)(content, opt, self.string_options.get(i).unwrap(), i)
                    .map(|score| (i, score))
            })
            .collect::<Vec<(usize, i64)>>();

        // sort the options by the score
        options.sort_unstable_by_key(|(_idx, score)| Reverse(*score));

        let new_scored_options = options.iter().map(|(idx, _)| *idx).collect::<Vec<usize>>();

        if self.scored_options == new_scored_options {
            return;
        }

        self.scored_options = new_scored_options;

        if self.config.reset_cursor {
            let _ = self.update_cursor_position(0);
        } else if self.scored_options.len() <= self.cursor_index {
            let _ = self.update_cursor_position(self.scored_options.len().saturating_sub(1));
        }
    }
}

impl<'a, Backend, T> Prompt<Backend> for MultiCountPrompt<'a, T>
where
    Backend: MultiCountBackend,
    T: Display,
{
    type Config = MultiCountConfig;
    type InnerAction = MultiCountPromptAction;
    type Output = Vec<CountedListOption<T>>;
    fn message(&self) -> &str {
        self.message
    }
    fn config(&self) -> &MultiCountConfig {
        &self.config
    }
    fn format_answer(&self, answer: &Vec<CountedListOption<T>>) -> String {
        let refs: Vec<CountedListOption<&T>> =
            answer.iter().map(CountedListOption::as_ref).collect();
        (self.formatter)(&refs)
    }

    fn setup(&mut self) -> InquireResult<()> {
        self.run_scorer();
        Ok(())
    }

    fn submit(&mut self) -> InquireResult<Option<Vec<CountedListOption<T>>>> {
        let answer = match self.validate_current_answer()? {
            Validation::Valid => Some(self.get_final_answer()),
            Validation::Invalid(msg) => {
                self.error = Some(msg);
                None
            }
        };

        Ok(answer)
    }

    /// appears to be the implementation of the actual actions written in action.rs
    fn handle(&mut self, action: MultiCountPromptAction) -> InquireResult<ActionResult> {
        let result = match action {
            MultiCountPromptAction::MoveUp => self.move_cursor_up(1, true),
            MultiCountPromptAction::MoveDown => self.move_cursor_down(1, true),
            MultiCountPromptAction::PageUp => self.move_cursor_up(self.config.page_size, false),
            MultiCountPromptAction::PageDown => self.move_cursor_down(self.config.page_size, false),
            MultiCountPromptAction::MoveToStart => self.move_cursor_up(usize::MAX, false),
            MultiCountPromptAction::MoveToEnd => self.move_cursor_down(usize::MAX, false),
            //MultiCountPromptAction::ToggleCurrentOption => self.toggle_cursor_selection(),
            MultiCountPromptAction::Increment => self.alter_value(1),
            MultiCountPromptAction::Decrement => self.alter_value(-1),
            MultiCountPromptAction::MultiIncrement(qty) => self.alter_value(qty as i32),
            MultiCountPromptAction::MultiDecrement(qty) => self.alter_value(-(qty as i32)),
            MultiCountPromptAction::SetCountCurrentOption(qty) => self.set_value(qty),
            MultiCountPromptAction::ClearSelections => {
                self.counts.clear();
                ActionResult::NeedsRedraw
            }
            MultiCountPromptAction::FilterInput(input_action) => match self.input.as_mut() {
                Some(input) => {
                    let result = input.handle(input_action);

                    if let InputActionResult::ContentChanged = result {
                        self.run_scorer();
                    }

                    result.into()
                }
                None => ActionResult::Clean,
            },
        };

        let result = self.clear_input_if_needed(action).merge(result);

        Ok(result)
    }

    fn render(&self, backend: &mut Backend) -> InquireResult<()> {
        let prompt = &self.message;

        if let Some(err) = &self.error {
            backend.render_error_message(err)?;
        }

        backend.render_multiselect_prompt(prompt, self.input.as_ref())?;

        let choices = self
            .scored_options
            .iter()
            .cloned()
            .map(|i| ListOption::new(i, self.options.get(i).unwrap()))
            .collect::<Vec<ListOption<&T>>>();

        let page = paginate(self.config.page_size, &choices, Some(self.cursor_index));

        backend.render_options(page, &self.counts)?;

        if let Some(help_message) = self.help_message {
            backend.render_help_message(help_message)?;
        }

        Ok(())
    }
}
