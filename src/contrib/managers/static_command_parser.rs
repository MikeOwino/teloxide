use crate::{
    contrib::parser::{DataWithUWC, Parser},
    prelude::UpdateWithCx,
    types::Message,
};

pub struct StaticCommandParserBuilder {
    prefix: Option<String>,
    command: String,
}
impl StaticCommandParserBuilder {
    pub fn new<T: Into<String>>(command: T) -> StaticCommandParserBuilder {
        Self { prefix: None, command: command.into() }
    }
    pub fn prefix<T: Into<String>>(mut self, prefix: T) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
    pub fn build(self) -> StaticCommandParser {
        let prefix = match self.prefix {
            Some(p) => p,
            None => "/".to_string(),
        };
        StaticCommandParser { command: format!("{}{}", prefix, self.command) }
    }
}

/// Represents a parser for static bot commands. Static means without args. It
/// may be, for example, `/start`, `/help` commands.
///
/// Example:
/// ```
/// use teloxide::{
///     contrib::{managers::StaticCommandParser, parser::Parser},
///     dummies::{text_message, update_with_cx},
/// };
///
/// let parser = StaticCommandParser::init("/start");
///
/// let start = update_with_cx(text_message("/start"));
/// assert!(parser.parse(start).is_ok());
///
/// let help = update_with_cx(text_message("/help"));
/// assert!(parser.parse(help).is_err());
/// ```
#[derive(Debug)]
pub struct StaticCommandParser {
    command: String,
}

impl StaticCommandParser {
    pub fn init<T: Into<String>>(command: T) -> Self {
        Self { command: command.into() }
    }
}

impl Parser for StaticCommandParser {
    type Update = Message;
    type Output = ();

    fn parse(
        &self,
        cx: UpdateWithCx<Self::Update>,
    ) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        let text = match cx.update.text() {
            Some(t) => t,
            None => return Err(cx),
        };
        match text == self.command {
            true => Ok(DataWithUWC::new((), cx)),
            false => Err(cx),
        }
    }
}