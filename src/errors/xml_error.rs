use std::fmt;

#[derive(Debug)]
pub enum XmlError {
	InvalidBool { element: &'static str, value: String },
	MissingElement(&'static str),
	MissingText(&'static str),
	Parse(roxmltree::Error),
}

impl fmt::Display for XmlError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidBool { element, value } => {
				write!(f, "invalid boolean value for {element}: {value}")
			}
			Self::MissingElement(element) => write!(f, "missing required element: {element}"),
			Self::MissingText(element) => write!(f, "missing text for element: {element}"),
			Self::Parse(err) => write!(f, "xml parse error: {err}"),
		}
	}
}

impl std::error::Error for XmlError {}

impl From<roxmltree::Error> for XmlError {
	fn from(value: roxmltree::Error) -> Self {
		Self::Parse(value)
	}
}

