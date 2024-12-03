use std::fmt;
use std::path::PathBuf;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    TrueFalseSymbol {
        filename: PathBuf,
        location: Location,
    },
    AnyIsNa {
        filename: PathBuf,
        location: Location,
    },
    AnyDuplicated {
        filename: PathBuf,
        location: Location,
    },
}

impl Message {
    /// Short ID for the message.
    pub fn code(&self) -> &'static str {
        match self {
            Message::TrueFalseSymbol { .. } => "T-F-symbols",
            Message::AnyIsNa { .. } => "any-na",
            Message::AnyDuplicated { .. } => "any-duplicated",
        }
    }

    /// The body text for the message.
    pub fn body(&self) -> &'static str {
        match self {
            Message::TrueFalseSymbol { .. } => "`T` and `F` can be confused with variable names. Spell `TRUE` and `FALSE` entirely instead.",
            Message::AnyIsNa { .. } => "`any(is.na(...))` is inefficient. Use `anyNA(...)` instead.",
            Message::AnyDuplicated { .. } => "`any(duplicated(...))` is inefficient. Use `anyDuplicated(...) > 0` instead.",
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::AnyDuplicated { filename, location }
            | Message::AnyIsNa { filename, location }
            | Message::TrueFalseSymbol { filename, location } => write!(
                f,
                "{} [{}:{}] {} {}",
                filename.to_string_lossy().white().bold(),
                location.row,
                location.column,
                self.code().red().bold(),
                self.body()
            ),
        }
    }
}
