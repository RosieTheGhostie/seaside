pub mod error_code;
pub mod label;
pub mod note;
pub mod span;

pub use error_code::{ErrorCode, ToErrorCode};
pub use label::Label;
pub use note::Note;
pub use span::Span;

use std::{borrow::Cow, path::Path};

#[derive(Clone, Debug, PartialEq)]
pub struct RichError {
    pub code: ErrorCode,
    pub message: String,
    pub broad_span: Span,
    pub label: Option<Label>,
    pub notes: Vec<Note>,
}

pub type RichResult<T> = Result<T, RichError>;

impl RichError {
    pub fn new<E>(err: E, broad_span: Span) -> Self
    where
        E: ToErrorCode + ToString,
    {
        Self {
            code: err.code(),
            message: err.to_string(),
            broad_span,
            label: None,
            notes: Vec::new(),
        }
    }

    pub fn with_note<S>(mut self, message: S) -> Self
    where
        S: ToString,
    {
        self.notes.push(Note::new_note(message));
        self
    }

    pub fn with_help<S>(mut self, message: S) -> Self
    where
        S: ToString,
    {
        self.notes.push(Note::new_help(message));
        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_narrow_span(mut self, span: Span) -> Self {
        let new_label = Label::new(span);
        self.label = Some(match self.label {
            Some(label) => new_label.with_message(label.message),
            None => new_label,
        });
        self
    }

    pub fn report<P>(self, source: &str, source_path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let source_name = match source_path.as_ref().file_name() {
            Some(name) => name.to_string_lossy(),
            None => Cow::Owned("<source>".to_string()),
        };

        let mut builder = ariadne::Report::build(
            ariadne::ReportKind::Error,
            (source_name.clone(), self.broad_span),
        )
        .with_config(ariadne::Config::new())
        .with_code(self.code)
        .with_message(self.message);

        for note in self.notes {
            note.add_to(&mut builder);
        }

        if let Some(label) = self.label {
            label.add_to(&mut builder, source_name.clone());
        }

        builder
            .finish()
            .eprint((source_name, ariadne::Source::from(source)))
    }
}
