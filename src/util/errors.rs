use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum TrieError {
    InvalidCharacter,
}

impl Error for TrieError {}

impl Display for TrieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrieError::InvalidCharacter => write!(f, "Invalid ASCII character."),
        }
    }
}
