use std::{collections::BTreeSet, fmt::Display, io::Result};

use unicode_width::UnicodeWidthStr;

use crate::{
    error::InquireResult,
    input::Input,
    list_option::ListOption,
    terminal::Terminal,
    ui::{IndexPrefix, Key, RenderConfig, Styled},
    utils::{int_log10, Page},
    validator::ErrorMessage,
};

use super::{frame_renderer::FrameRenderer, InputReader};

pub trait CommonBackend: InputReader {
    fn frame_setup(&mut self) -> Result<()>;
    fn frame_finish(&mut self, is_last_frame: bool) -> Result<()>;

    fn render_canceled_prompt(&mut self, prompt: &str) -> Result<()>;
    fn render_prompt_with_answer(&mut self, prompt: &str, answer: &str) -> Result<()>;

    fn render_error_message(&mut self, error: &ErrorMessage) -> Result<()>;
    fn render_help_message(&mut self, help: &str) -> Result<()>;
}

pub trait TextBackend: CommonBackend {
    fn render_prompt(
        &mut self,
        prompt: &str,
        default: Option<&str>,
        cur_input: &Input,
    ) -> Result<()>;
    fn render_suggestions<D: Display>(&mut self, page: Page<'_, ListOption<D>>) -> Result<()>;
}

#[cfg(feature = "editor")]
pub trait EditorBackend: CommonBackend {
    fn render_prompt(&mut self, prompt: &str, editor_command: &str) -> Result<()>;
}

pub trait SelectBackend: CommonBackend {
    fn render_select_prompt(&mut self, prompt: &str, cur_input: Option<&Input>) -> Result<()>;
    fn render_options<D: Display>(&mut self, page: Page<'_, ListOption<D>>) -> Result<()>;
}

pub trait MultiSelectBackend: CommonBackend {
    fn render_multiselect_prompt(&mut self, prompt: &str, cur_input: Option<&Input>) -> Result<()>;
    fn render_options<D: Display>(
        &mut self,
        page: Page<'_, ListOption<D>>,
        checked: &BTreeSet<usize>,
    ) -> Result<()>;
}

pub trait CustomTypeBackend: CommonBackend {
    fn render_prompt(
        &mut self,
        prompt: &str,
        default: Option<&str>,
        cur_input: &Input,
    ) -> Result<()>;
}

pub trait PasswordBackend: CommonBackend {
    fn render_prompt(&mut self, prompt: &str) -> Result<()>;
    fn render_prompt_with_masked_input(&mut self, prompt: &str, cur_input: &Input) -> Result<()>;
    fn render_prompt_with_unmasked_last_char_input(
        &mut self,
        prompt: &str,
        cur_input: &Input,
    ) -> Result<()>;
    fn render_prompt_with_full_input(&mut self, prompt: &str, cur_input: &Input) -> Result<()>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

pub struct Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    frame_renderer: FrameRenderer<T>,
    input_reader: I,
    render_config: RenderConfig<'a>,
}

impl<'a, I, T> Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    #[allow(clippy::large_types_passed_by_value)]
    pub fn new(input_reader: I, terminal: T, render_config: RenderConfig<'a>) -> Result<Self> {
        let backend = Self {
            frame_renderer: FrameRenderer::new(terminal)?,
            input_reader,
            render_config,
        };

        Ok(backend)
    }

    fn print_option_prefix<D: Display>(
        &mut self,
        option_relative_index: usize,
        page: &Page<'_, ListOption<D>>,
    ) -> Result<()> {
        let empty_prefix = Styled::new(" ");

        let x = if page.cursor == Some(option_relative_index) {
            self.render_config.highlighted_option_prefix
        } else if option_relative_index == 0 && !page.first {
            self.render_config.scroll_up_prefix
        } else if (option_relative_index + 1) == page.content.len() && !page.last {
            self.render_config.scroll_down_prefix
        } else {
            empty_prefix
        };

        self.frame_renderer.write_styled(x)
    }

    fn print_option_value<D: Display>(
        &mut self,
        option_relative_index: usize,
        option: &ListOption<D>,
        page: &Page<'_, ListOption<D>>,
    ) -> Result<()> {
        let stylesheet = if let Some(selected_option_style) = self.render_config.selected_option {
            match page.cursor {
                Some(cursor) if cursor == option_relative_index => selected_option_style,
                _ => self.render_config.option,
            }
        } else {
            self.render_config.option
        };

        self.frame_renderer
            .write_styled(Styled::new(&option.value).with_style_sheet(stylesheet))
    }

    fn print_option_index_prefix(&mut self, index: usize, max_index: usize) -> Option<Result<()>> {
        let index = index.saturating_add(1);

        let content = match self.render_config.option_index_prefix {
            IndexPrefix::None => None,
            IndexPrefix::Simple => Some(format!("{index})")),
            IndexPrefix::SpacePadded => {
                let width = int_log10(max_index.saturating_add(1));
                Some(format!("{index:width$})"))
            }
            IndexPrefix::ZeroPadded => {
                let width = int_log10(max_index.saturating_add(1));
                Some(format!("{index:0width$})"))
            }
        };

        content.map(|prefix| {
            self.frame_renderer
                .write_styled(Styled::new(prefix).with_style_sheet(self.render_config.option))
        })
    }

    fn print_default_value(&mut self, value: &str) -> Result<()> {
        let content = format!("({value})");
        let token = Styled::new(content).with_style_sheet(self.render_config.default_value);

        self.frame_renderer.write_styled(token)
    }

    fn print_prompt_with_prefix(&mut self, prefix: Styled<&str>, prompt: &str) -> Result<()> {
        self.frame_renderer.write_styled(prefix)?;

        self.frame_renderer.write(" ")?;

        self.frame_renderer
            .write_styled(Styled::new(prompt).with_style_sheet(self.render_config.prompt))?;

        Ok(())
    }

    fn print_prompt(&mut self, prompt: &str) -> Result<()> {
        self.print_prompt_with_prefix(self.render_config.prompt_prefix, prompt)
    }

    fn print_input(&mut self, input: &Input) -> Result<()> {
        self.frame_renderer.write(" ")?;

        // The cursor is at the beginning of the input line.
        // From here it's easier to mark the wanted cursor position
        // (based on the underlying input struct), as it's a simple
        // cur_pos + offset calculation.
        self.frame_renderer
            .mark_cursor_position(input.pre_cursor().width() as isize);

        if input.is_empty() {
            match input.placeholder() {
                Some("") | None => {}
                Some(p) => self.frame_renderer.write_styled(
                    Styled::new(p).with_style_sheet(self.render_config.placeholder),
                )?,
            }
        } else {
            self.frame_renderer.write_styled(
                Styled::new(input.content()).with_style_sheet(self.render_config.text_input),
            )?;
        }

        // if cursor is at end of input, we need to add
        // a space, otherwise the cursor will render on the
        // \n character, on the next line.
        if input.cursor() == input.length() {
            self.frame_renderer.write(' ')?;
        }

        Ok(())
    }

    fn print_prompt_with_input(
        &mut self,
        prompt: &str,
        default: Option<&str>,
        input: &Input,
    ) -> Result<()> {
        self.print_prompt(prompt)?;

        if let Some(default) = default {
            self.frame_renderer.write(" ")?;
            self.print_default_value(default)?;
        }

        self.print_input(input)?;

        self.new_line()?;

        Ok(())
    }

    fn new_line(&mut self) -> Result<()> {
        self.frame_renderer.write("\n")?;
        Ok(())
    }
}

impl<'a, I, T> CommonBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn frame_setup(&mut self) -> Result<()> {
        self.frame_renderer.start_frame()
    }

    fn frame_finish(&mut self, is_last_frame: bool) -> Result<()> {
        self.frame_renderer.finish_current_frame(is_last_frame)
    }

    fn render_canceled_prompt(&mut self, prompt: &str) -> Result<()> {
        self.print_prompt(prompt)?;

        self.frame_renderer.write(" ")?;

        self.frame_renderer
            .write_styled(self.render_config.canceled_prompt_indicator)?;

        self.new_line()?;

        Ok(())
    }

    fn render_prompt_with_answer(&mut self, prompt: &str, answer: &str) -> Result<()> {
        self.print_prompt_with_prefix(self.render_config.answered_prompt_prefix, prompt)?;

        self.frame_renderer.write(" ")?;

        let token = Styled::new(answer).with_style_sheet(self.render_config.answer);
        self.frame_renderer.write_styled(token)?;

        self.new_line()?;

        Ok(())
    }

    fn render_error_message(&mut self, error: &ErrorMessage) -> Result<()> {
        self.frame_renderer
            .write_styled(self.render_config.error_message.prefix)?;

        self.frame_renderer.write_styled(
            Styled::new(" ").with_style_sheet(self.render_config.error_message.separator),
        )?;

        let message = match error {
            ErrorMessage::Default => self.render_config.error_message.default_message,
            ErrorMessage::Custom(msg) => msg,
        };

        self.frame_renderer.write_styled(
            Styled::new(message).with_style_sheet(self.render_config.error_message.message),
        )?;

        self.new_line()?;

        Ok(())
    }

    fn render_help_message(&mut self, help: &str) -> Result<()> {
        self.frame_renderer
            .write_styled(Styled::new("[").with_style_sheet(self.render_config.help_message))?;

        self.frame_renderer
            .write_styled(Styled::new(help).with_style_sheet(self.render_config.help_message))?;

        self.frame_renderer
            .write_styled(Styled::new("]").with_style_sheet(self.render_config.help_message))?;

        self.new_line()?;

        Ok(())
    }
}

impl<'a, I, T> TextBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn render_prompt(
        &mut self,
        prompt: &str,
        default: Option<&str>,
        cur_input: &Input,
    ) -> Result<()> {
        self.print_prompt_with_input(prompt, default, cur_input)
    }

    fn render_suggestions<D: Display>(&mut self, page: Page<'_, ListOption<D>>) -> Result<()> {
        for (idx, option) in page.content.iter().enumerate() {
            self.print_option_prefix(idx, &page)?;

            self.frame_renderer.write(" ")?;
            self.print_option_value(idx, option, &page)?;

            self.new_line()?;
        }

        Ok(())
    }
}

#[cfg(feature = "editor")]
impl<'a, I, T> EditorBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn render_prompt(&mut self, prompt: &str, editor_command: &str) -> Result<()> {
        self.print_prompt(prompt)?;

        self.frame_renderer.write(" ")?;

        let message = format!("[(e) to open {}, (enter) to submit]", editor_command);
        let token = Styled::new(message).with_style_sheet(self.render_config.editor_prompt);
        self.frame_renderer.write_styled(token)?;

        self.new_line()?;

        Ok(())
    }
}

impl<'a, I, T> SelectBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn render_select_prompt(&mut self, prompt: &str, cur_input: Option<&Input>) -> Result<()> {
        if let Some(input) = cur_input {
            self.print_prompt_with_input(prompt, None, input)
        } else {
            self.print_prompt(prompt)
        }
    }

    fn render_options<D: Display>(&mut self, page: Page<'_, ListOption<D>>) -> Result<()> {
        for (idx, option) in page.content.iter().enumerate() {
            self.print_option_prefix(idx, &page)?;

            self.frame_renderer.write(" ")?;

            if let Some(res) = self.print_option_index_prefix(option.index, page.total) {
                res?;
                self.frame_renderer.write(" ")?;
            }

            self.print_option_value(idx, option, &page)?;

            self.new_line()?;
        }

        Ok(())
    }
}

impl<'a, I, T> MultiSelectBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn render_multiselect_prompt(&mut self, prompt: &str, cur_input: Option<&Input>) -> Result<()> {
        if let Some(input) = cur_input {
            self.print_prompt_with_input(prompt, None, input)
        } else {
            self.print_prompt(prompt)
        }
    }

    fn render_options<D: Display>(
        &mut self,
        page: Page<'_, ListOption<D>>,
        checked: &BTreeSet<usize>,
    ) -> Result<()> {
        for (idx, option) in page.content.iter().enumerate() {
            self.print_option_prefix(idx, &page)?;

            self.frame_renderer.write(" ")?;

            if let Some(res) = self.print_option_index_prefix(option.index, page.total) {
                res?;
                self.frame_renderer.write(" ")?;
            }

            let mut checkbox = match checked.contains(&option.index) {
                true => self.render_config.selected_checkbox,
                false => self.render_config.unselected_checkbox,
            };

            match (self.render_config.selected_option, page.cursor) {
                (Some(stylesheet), Some(cursor)) if cursor == idx => checkbox.style = stylesheet,
                _ => {}
            }

            self.frame_renderer.write_styled(checkbox)?;

            self.frame_renderer.write(" ")?;

            self.print_option_value(idx, option, &page)?;

            self.new_line()?;
        }

        Ok(())
    }
}

#[cfg(feature = "date")]
pub mod date {
    use std::{io::Result, ops::Sub};

    use chrono::{Datelike, Duration};

    use crate::{
        date_utils::get_start_date,
        terminal::Terminal,
        ui::{InputReader, Styled},
    };

    use super::{Backend, CommonBackend};

    pub trait DateSelectBackend: CommonBackend {
        fn render_calendar_prompt(&mut self, prompt: &str) -> Result<()>;

        #[allow(clippy::too_many_arguments)]
        fn render_calendar(
            &mut self,
            month: chrono::Month,
            year: i32,
            week_start: chrono::Weekday,
            today: chrono::NaiveDate,
            selected_date: chrono::NaiveDate,
            min_date: Option<chrono::NaiveDate>,
            max_date: Option<chrono::NaiveDate>,
        ) -> Result<()>;
    }

    impl<'a, I, T> DateSelectBackend for Backend<'a, I, T>
    where
        I: InputReader,
        T: Terminal,
    {
        fn render_calendar_prompt(&mut self, prompt: &str) -> Result<()> {
            self.print_prompt(prompt)?;
            self.new_line()?;
            Ok(())
        }

        fn render_calendar(
            &mut self,
            month: chrono::Month,
            year: i32,
            week_start: chrono::Weekday,
            today: chrono::NaiveDate,
            selected_date: chrono::NaiveDate,
            min_date: Option<chrono::NaiveDate>,
            max_date: Option<chrono::NaiveDate>,
        ) -> Result<()> {
            macro_rules! write_prefix {
                () => {{
                    self.frame_renderer
                        .write_styled(self.render_config.calendar.prefix)?;
                    self.frame_renderer.write(" ")
                }};
            }

            // print header (month year)
            let header = format!("{} {}", month.name().to_lowercase(), year);
            let header = format!("{header:^20}");
            let header = Styled::new(header).with_style_sheet(self.render_config.calendar.header);

            write_prefix!()?;

            self.frame_renderer.write_styled(header)?;

            self.new_line()?;

            // print week header
            let mut current_weekday = week_start;
            let mut week_days: Vec<String> = vec![];
            for _ in 0..7 {
                let mut formatted = format!("{current_weekday}");
                formatted.make_ascii_lowercase();
                formatted.pop();
                week_days.push(formatted);

                current_weekday = current_weekday.succ();
            }

            let week_days = Styled::new(week_days.join(" "))
                .with_style_sheet(self.render_config.calendar.week_header);

            write_prefix!()?;

            self.frame_renderer.write_styled(week_days)?;
            self.new_line()?;

            // print dates
            let mut date_it = get_start_date(month, year);
            // first date of week-line is possibly in the previous month
            if date_it.weekday() == week_start {
                date_it = date_it.sub(
                    Duration::try_weeks(1).expect("overflow when calculating duration of 1 week"),
                );
            } else {
                while date_it.weekday() != week_start {
                    date_it = match date_it.pred_opt() {
                        Some(date) => date,
                        None => break,
                    };
                }
            }

            for _ in 0..6 {
                write_prefix!()?;

                for i in 0..7 {
                    if i > 0 {
                        self.frame_renderer.write(" ")?;
                    }

                    let date = format!("{:2}", date_it.day());

                    let cursor_offset = if date_it.day() < 10 { 1 } else { 0 };

                    let mut style_sheet = crate::ui::StyleSheet::empty();

                    if date_it == selected_date {
                        self.frame_renderer.mark_cursor_position(cursor_offset);
                        if let Some(custom_style_sheet) = self.render_config.calendar.selected_date
                        {
                            style_sheet = custom_style_sheet;
                        }
                    } else if date_it == today {
                        style_sheet = self.render_config.calendar.today_date;
                    } else if date_it.month() != month.number_from_month() {
                        style_sheet = self.render_config.calendar.different_month_date;
                    }

                    if let Some(min_date) = min_date {
                        if date_it < min_date {
                            style_sheet = self.render_config.calendar.unavailable_date;
                        }
                    }

                    if let Some(max_date) = max_date {
                        if date_it > max_date {
                            style_sheet = self.render_config.calendar.unavailable_date;
                        }
                    }

                    let token = Styled::new(date).with_style_sheet(style_sheet);
                    self.frame_renderer.write_styled(token)?;

                    date_it = date_it.succ_opt().unwrap_or(date_it);
                }

                self.new_line()?;
            }

            Ok(())
        }
    }
}

impl<'a, I, T> CustomTypeBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn render_prompt(
        &mut self,
        prompt: &str,
        default: Option<&str>,
        cur_input: &Input,
    ) -> Result<()> {
        self.print_prompt_with_input(prompt, default, cur_input)
    }
}

impl<'a, I, T> PasswordBackend for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn render_prompt(&mut self, prompt: &str) -> Result<()> {
        self.print_prompt(prompt)?;
        self.new_line()?;
        Ok(())
    }

    fn render_prompt_with_masked_input(&mut self, prompt: &str, cur_input: &Input) -> Result<()> {
        let masked_string: String = (0..cur_input.length())
            .map(|_| self.render_config.password_mask)
            .collect();

        let masked_input = Input::new_with(masked_string).with_cursor(cur_input.cursor());

        self.print_prompt_with_input(prompt, None, &masked_input)
    }

    fn render_prompt_with_unmasked_last_char_input(
        &mut self,
        prompt: &str,
        cur_input: &Input,
    ) -> Result<()> {
        let masked_string: String = (0..cur_input.length().saturating_sub(1))
            .map(|_| self.render_config.password_mask)
            .chain(cur_input.content().chars().rev().take(1))
            .collect();

        let masked_input = Input::new_with(masked_string).with_cursor(cur_input.cursor());

        self.print_prompt_with_input(prompt, None, &masked_input)
    }

    fn render_prompt_with_full_input(&mut self, prompt: &str, cur_input: &Input) -> Result<()> {
        self.print_prompt_with_input(prompt, None, cur_input)
    }
}

impl<'a, I, T> InputReader for Backend<'a, I, T>
where
    I: InputReader,
    T: Terminal,
{
    fn read_key(&mut self) -> InquireResult<Key> {
        self.input_reader.read_key()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::collections::VecDeque;

    use chrono::{Month, NaiveDate, Weekday};

    use crate::{
        input::Input,
        ui::{InputReader, Key},
        validator::ErrorMessage,
    };

    use super::{CommonBackend, CustomTypeBackend};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        Prompt(String),
        DefaultValue(String),
        Input(Input),
        CanceledPrompt(String),
        AnsweredPrompt(String, String),
        ErrorMessage(ErrorMessage),
        HelpMessage(String),
        Calendar {
            month: Month,
            year: i32,
            week_start: Weekday,
            today: NaiveDate,
            selected_date: NaiveDate,
            min_date: Option<NaiveDate>,
            max_date: Option<NaiveDate>,
        },
        PromptEnd,
    }

    #[derive(Default, Debug, Clone)]
    pub struct Frame {
        content: Vec<Token>,
    }

    impl Frame {
        pub fn has_token(&self, token: &Token) -> bool {
            self.content.iter().any(|t| t == token)
        }

        pub fn tokens(&self) -> &[Token] {
            &self.content
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct FakeBackend {
        pub input: VecDeque<Key>,
        pub frames: Vec<Frame>,
        pub cur_frame: Option<Frame>,
    }

    impl FakeBackend {
        pub fn new(input: Vec<Key>) -> Self {
            Self {
                input: input.into(),
                frames: vec![],
                cur_frame: None,
            }
        }

        fn push_token(&mut self, token: Token) {
            if let Some(frame) = self.cur_frame.as_mut() {
                frame.content.push(token);
            } else {
                panic!("No frame to push token");
            }
        }
        pub fn frames(&self) -> &[Frame] {
            &self.frames
        }
    }

    impl InputReader for FakeBackend {
        fn read_key(&mut self) -> crate::error::InquireResult<Key> {
            self.input
                .pop_front()
                .ok_or(crate::error::InquireError::IO(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "No more keys in input",
                )))
        }
    }

    impl CommonBackend for FakeBackend {
        fn frame_setup(&mut self) -> std::io::Result<()> {
            self.cur_frame = Some(Frame::default());
            Ok(())
        }

        fn frame_finish(&mut self, is_last_frame: bool) -> std::io::Result<()> {
            if is_last_frame {
                self.push_token(Token::PromptEnd);
            }

            if let Some(frame) = self.cur_frame.take() {
                self.frames.push(frame);
            } else {
                panic!("No frame to finish");
            }
            Ok(())
        }

        fn render_canceled_prompt(&mut self, prompt: &str) -> std::io::Result<()> {
            self.push_token(Token::CanceledPrompt(prompt.to_string()));
            Ok(())
        }

        fn render_prompt_with_answer(&mut self, prompt: &str, answer: &str) -> std::io::Result<()> {
            self.push_token(Token::AnsweredPrompt(
                prompt.to_string(),
                answer.to_string(),
            ));
            Ok(())
        }

        fn render_error_message(&mut self, error: &ErrorMessage) -> std::io::Result<()> {
            self.push_token(Token::ErrorMessage(error.clone()));
            Ok(())
        }

        fn render_help_message(&mut self, help: &str) -> std::io::Result<()> {
            self.push_token(Token::HelpMessage(help.to_string()));
            Ok(())
        }
    }

    #[cfg(feature = "date")]
    impl crate::ui::date::DateSelectBackend for FakeBackend {
        fn render_calendar_prompt(&mut self, prompt: &str) -> std::io::Result<()> {
            self.push_token(Token::Prompt(prompt.to_string()));
            Ok(())
        }

        fn render_calendar(
            &mut self,
            month: Month,
            year: i32,
            week_start: Weekday,
            today: NaiveDate,
            selected_date: NaiveDate,
            min_date: Option<NaiveDate>,
            max_date: Option<NaiveDate>,
        ) -> std::io::Result<()> {
            self.push_token(Token::Calendar {
                month,
                year,
                week_start,
                today,
                selected_date,
                min_date,
                max_date,
            });
            Ok(())
        }
    }

    impl CustomTypeBackend for FakeBackend {
        fn render_prompt(
            &mut self,
            prompt: &str,
            default: Option<&str>,
            cur_input: &Input,
        ) -> std::io::Result<()> {
            self.push_token(Token::Prompt(prompt.to_string()));
            if let Some(default) = default {
                self.push_token(Token::DefaultValue(default.to_string()));
            }
            self.push_token(Token::Input(cur_input.clone()));
            Ok(())
        }
    }
}
