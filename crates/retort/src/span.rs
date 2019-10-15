use {
    crate::style::Stylesheet,
    std::{io, ops},
    termcolor::WriteColor,
};

/// A span of a snippet to be annotated.
///
/// Cloning is expected to be cheap.
pub trait Span: Clone {
    /// A position within the span. Positions must sort
    /// correctly for every `Span` from the same origin.
    ///
    /// For most spans, this will be a `usize` index
    /// or a `(usize, usize)` line/column pair.
    type Pos: PartialOrd;

    /// The start position of the span.
    ///
    /// This is expected to be similar in cost to a field access.
    fn start(&self) -> Self::Pos;

    /// The end position of the span.
    ///
    /// This is expected to be similar in cost to a field access.
    fn end(&self) -> Self::Pos;

    // FIXME: Can this be removed? `count_chars(Pos, Pos)` instead?
    /// Create a new span with positions from a span of the same origin.
    fn new(&self, start: Self::Pos, end: Self::Pos) -> Self;
}

/// A type to resolve spans from opaque spans to information required for annotation.
pub trait SpanResolver<Sp> {
    /// Write the span to a [`WriteColor`] sink.
    ///
    /// When calling `write_span`, the writer is styled with the base style.
    /// Style can be customized manually or by proxying through the stylesheet.
    fn write_span(
        &mut self,
        w: &mut dyn WriteColor,
        stylesheet: &mut dyn Stylesheet,
        span: &Sp,
    ) -> io::Result<()>;

    /// Count the number of characters wide this span is in a terminal font.
    fn count_chars(&mut self, span: &Sp) -> usize;

    /// Get the first line in a span. The line includes the whole line,
    /// even if that extends out of the source span being iterated.
    ///
    /// If the input span is empty, the line it is on is produced.
    fn first_line_of(&mut self, span: &Sp) -> SpannedLine<Sp>;
    /// Get the next line in a span. The line includes the whole line,
    /// even if that extends out of the source span being iterated.
    ///
    /// If the next line does not overlap the span at all, `None` is produced.
    fn next_line_of(&mut self, span: &Sp, previous: SpannedLine<Sp>) -> Option<SpannedLine<Sp>>;
}

/// A reference to a line within a snippet.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpannedLine<Sp> {
    /// The span of the line, _not_ including the terminating newline (if present).
    pub span: Sp,
    /// The line number.
    pub num: usize,
}

impl Span for ops::Range<usize> {
    type Pos = usize;

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn new(&self, start: usize, end: usize) -> Self {
        start..end
    }
}

impl SpanResolver<ops::Range<usize>> for &'_ str {
    fn write_span(
        &mut self,
        w: &mut dyn WriteColor,
        _stylesheet: &mut dyn Stylesheet,
        span: &ops::Range<usize>,
    ) -> io::Result<()> {
        write!(w, "{}", &self[span.start..span.end])
    }

    fn count_chars(&mut self, span: &ops::Range<usize>) -> usize {
        bytecount::num_chars(self[span.start..span.end].as_bytes())
    }

    fn first_line_of(&mut self, span: &ops::Range<usize>) -> SpannedLine<ops::Range<usize>> {
        // Find the position after the newline immediately prior to this span.
        let start = self[..span.start].rfind('\n').map_or(0, |i| i + 1);
        // Find the position before the next newline.
        let end = self[start..].find('\n').map_or(self.len(), |i| i + start);
        SpannedLine {
            span: start..end,
            num: bytecount::count(self[..start].as_bytes(), b'\n') + 1,
        }
    }

    fn next_line_of(
        &mut self,
        span: &ops::Range<usize>,
        previous: SpannedLine<ops::Range<usize>>,
    ) -> Option<SpannedLine<ops::Range<usize>>> {
        // Find the position after the newline immediately following the previous span.
        self[previous.span.end..]
            .find('\n')
            .map(|i| i + 1 + previous.span.end)
            // If the start is within the full span,
            .filter(|&start| start < span.end)
            // Slice until the next newline.
            .map(|start| start..self[start..].find('\n').map_or(self.len(), |i| i + start))
            .map(|span| SpannedLine {
                span,
                num: previous.num + 1,
            })
    }
}
