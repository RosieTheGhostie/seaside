pub type Span = core::ops::Range<usize>;

/// Combines two spans into one span.
///
/// The resulting span will contain all indices in the original spans, possibly with some additional
/// indices included between them.
///
/// # Examples
///
/// ```
/// # use crate::rich::span::combine_spans;
/// assert_eq!(combine_spans(69..420, 42..100), 42..420);
/// assert_eq!(combine_spans(21..32, 27..84), 21..84);
/// assert_eq!(combine_spans(0..100, 300..500), 0..500);
/// assert_eq!(combine_spans(0..100, 10..20), 0..100);
/// ```
pub const fn combine_spans(spans: [Span; 2]) -> Span {
    Span {
        start: usize_min(spans[0].start, spans[1].start),
        end: usize_max(spans[0].end, spans[1].end),
    }
}

/// Combines the consumed span with the consumer.
///
/// The consumer will then contain all the indices it used to along with all those in the consumed
/// span (possibly plus some extras to make it one continuous span).
///
/// # Examples
///
/// ```
/// # use crate::rich::span::consume_span;
/// let mut total_span = 69..420;
/// consume_span(&mut total_span, 42..100);
/// assert_eq!(total_span, 42..420);
/// consume_span(&mut total_span, 420..696);
/// assert_eq!(total_span, 42..696);
/// consume_span(&mut total_span, 700..1000);
/// assert_eq!(total_span, 42..1000);
/// consume_span(&mut total_span, 100..200); // This shouldn't do anything.
/// assert_eq!(total_span, 42..1000);
/// ```
pub const fn consume_span(consumer: &mut Span, to_consume: Span) {
    *consumer = Span {
        start: usize_min(consumer.start, to_consume.start),
        end: usize_max(consumer.end, to_consume.end),
    };
}

/// Returns the minimum of two [`usize`]s.
///
/// This is a `const` alternative to [`usize::min`].
const fn usize_min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

/// Returns the maximum of two [`usize`]s.
///
/// This is a `const` alternative to [`usize::max`].
const fn usize_max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}
