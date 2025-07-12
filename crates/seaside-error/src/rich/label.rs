use super::span::Span;
use ariadne::ReportBuilder;

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    pub message: String,
    pub span: Span,
}

impl Label {
    pub const fn new(span: Span) -> Self {
        Self {
            message: String::new(),
            span,
        }
    }

    pub fn with_message<S>(mut self, message: S) -> Self
    where
        S: ToString,
    {
        self.message = message.to_string();
        self
    }

    pub fn add_to<Source>(
        self,
        builder: &mut ReportBuilder<'_, (Source, Span)>,
        source_name: Source,
    ) where
        (Source, Span): ariadne::Span,
    {
        let label = ariadne::Label::new((source_name, self.span)).with_color(ariadne::Color::Red);
        builder.add_label(if !self.message.is_empty() {
            label.with_message(self.message)
        } else {
            label
        });
    }
}
