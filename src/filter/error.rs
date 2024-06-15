use core::fmt;

use annotate_snippets::{Level, Renderer, Snippet};
use pest::{error::InputLocation, RuleType};

pub struct FilterError<R> {
    parsing_error: pest::error::Error<R>,
}

impl<R> fmt::Debug for FilterError<R>
where
    R: RuleType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let input = self.parsing_error.line();
        let pos = match self.parsing_error.location {
            InputLocation::Pos(pos) => pos..pos,
            InputLocation::Span((start, end)) => start..end,
        };

        let message = Level::Error.title("failed to parse filter").snippet(
            Snippet::source(input).annotation(Level::Error.span(pos).label("unexpected token")),
        );

        let renderer = Renderer::styled();
        let rendered = renderer.render(message);

        write!(f, "{}", rendered)
    }
}

impl<R> fmt::Display for FilterError<R>
where
    R: RuleType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<R> std::error::Error for FilterError<R> where R: RuleType {}

impl<R> From<pest::error::Error<R>> for Box<FilterError<R>>
where
    R: RuleType,
{
    fn from(value: pest::error::Error<R>) -> Self {
        match value.variant {
            pest::error::ErrorVariant::ParsingError {
                positives: _,
                negatives: _,
            } => Box::new(FilterError {
                parsing_error: value.clone(),
            }),
            _ => {
                unimplemented!()
            }
        }
    }
}
