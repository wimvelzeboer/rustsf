use crate::Error;
use std::fmt;

#[derive(Debug)]
pub enum XmlParseError {
	Xml(roxmltree::Error),
	MissingElement(&'static str),
	MissingText(&'static str),
	InvalidBool { element: &'static str, value: String },
}

impl fmt::Display for XmlParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Xml(err) => write!(f, "xml parse error: {err}"),
			Self::MissingElement(element) => write!(f, "missing required element: {element}"),
			Self::MissingText(element) => write!(f, "missing text for element: {element}"),
			Self::InvalidBool { element, value } => {
				write!(f, "invalid boolean value for {element}: {value}")
			}
		}
	}
}

impl std::error::Error for XmlParseError {}

impl From<roxmltree::Error> for XmlParseError {
	fn from(value: roxmltree::Error) -> Self {
		Self::Xml(value)
	}
}
