use super::span::Span;
use ariadne::ReportBuilder;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum NoteKind {
    #[default]
    Note,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub kind: NoteKind,
    pub message: String,
}

impl Note {
    pub fn new_note<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self {
            kind: NoteKind::Note,
            message: message.to_string(),
        }
    }

    pub fn new_help<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self {
            kind: NoteKind::Help,
            message: message.to_string(),
        }
    }

    pub fn add_to<Source>(self, builder: &mut ReportBuilder<'_, (Source, Span)>)
    where
        (Source, Span): ariadne::Span,
    {
        match self.kind {
            NoteKind::Note => builder.add_note(self.message),
            NoteKind::Help => builder.add_help(self.message),
        }
    }
}
