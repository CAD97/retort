use std::{fmt, io};
pub use termcolor::WriteColor;

pub mod diagnostic;
pub mod renderer;
pub mod style;

pub trait Span: fmt::Debug + Copy {
    type Origin: ?Sized + fmt::Debug + Eq;

    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn new(&self, start: usize, end: usize) -> Self;
    fn origin(&self) -> &Self::Origin;
}

pub trait SpanResolver<Sp> {
    fn first_line_of(&mut self, span: Sp) -> Option<SpannedLine<Sp>>;
    fn next_line_of(&mut self, span: Sp, line: SpannedLine<Sp>) -> Option<SpannedLine<Sp>>;
    fn write_span(&mut self, w: &mut dyn WriteColor, span: Sp) -> io::Result<()>;
    fn write_origin(&mut self, w: &mut dyn io::Write, span: Sp) -> io::Result<()>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpannedLine<Sp> {
    line_num: usize,
    char_count: usize,
    span: Sp,
}

impl Span for (usize, usize) {
    type Origin = ();

    fn start(&self) -> usize {
        self.0
    }
    fn end(&self) -> usize {
        self.1
    }
    fn new(&self, start: usize, end: usize) -> Self {
        (start, end)
    }
    fn origin(&self) -> &Self::Origin {
        &()
    }
}

impl<Sp: Span<Origin = ()>> Span for (&'_ str, Sp) {
    type Origin = str;

    fn start(&self) -> usize {
        self.1.start()
    }
    fn end(&self) -> usize {
        self.1.end()
    }
    fn new(&self, start: usize, end: usize) -> Self {
        (self.0, self.1.new(start, end))
    }
    fn origin(&self) -> &Self::Origin {
        self.0
    }
}

mod hidden {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct SubSpanIterator<'a, Sp: Span> {
        pub(super) span: Sp,
        pub(super) text: &'a str,
        pub(super) line_num: usize,
        pub(super) remaining_lines: usize,
    }

    impl<Sp: Span> Iterator for SubSpanIterator<'_, Sp> {
        type Item = (usize, Sp);

        fn next(&mut self) -> Option<(usize, Sp)> {
            if let Some(line) = self.text.lines().next() {
                let start = self.span.start();
                let end = start + line.len();
                let out = (self.line_num, self.span.new(start, end));
                let after_nl = self.text[line.len()..]
                    .find('\n')
                    .map_or(self.text.len(), |i| i + 1 + line.len());
                self.text = &self.text[after_nl..];
                self.span = self.span.new(start + after_nl, self.span.end());
                self.line_num += 1;
                self.remaining_lines -= 1;
                Some(out)
            } else {
                assert_eq!(self.remaining_lines, 0);
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.len(), Some(self.len()))
        }
    }

    impl<Sp: Span> ExactSizeIterator for SubSpanIterator<'_, Sp> {
        fn len(&self) -> usize {
            self.remaining_lines
        }
    }
}

#[allow(clippy::or_fun_call)] // Span::end is an accessor
fn slice_line_span<Sp: Span>(text: &str, span: Sp, start: usize) -> Option<SpannedLine<Sp>> {
    let end = text[start..span.end()]
        .find('\n')
        .map_or(span.end(), |i| i + start);
    if start != end {
        let line = &text[start..end];
        let span = span.new(start, end);
        let line_num = bytecount::count(text[..start].as_bytes(), b'\n') + 1;
        let char_count = bytecount::num_chars(line.as_bytes());
        Some(SpannedLine {
            line_num,
            char_count,
            span,
        })
    } else {
        None
    }
}

impl<'a, Sp: Span> SpanResolver<Sp> for &'a str
where
    Sp::Origin: fmt::Display,
{
    fn first_line_of(&mut self, span: Sp) -> Option<SpannedLine<Sp>> {
        let start = self[..span.start()].rfind('\n').map_or(0, |i| i + 1);
        slice_line_span(self, span, start)
    }

    fn next_line_of(&mut self, span: Sp, line: SpannedLine<Sp>) -> Option<SpannedLine<Sp>> {
        let start = self[line.span.end()..].find('\n').map_or(0, |i| i + 1) + line.span.end();
        slice_line_span(self, span, start)
    }

    fn write_span(&mut self, w: &mut dyn WriteColor, span: Sp) -> io::Result<()> {
        w.reset()?;
        write!(w, "{}", &self[span.start()..span.end()])
    }

    fn write_origin(&mut self, w: &mut dyn io::Write, span: Sp) -> io::Result<()> {
        let after_nl = self[..span.start()]
            .rfind('\n')
            .map_or(span.start(), |i| i + 1);
        let line_no = bytecount::count(self[..after_nl].as_bytes(), b'\n') + 1;
        let col_no = bytecount::num_chars(self[after_nl..span.start()].as_bytes()) + 1;
        write!(w, "{}:{}:{}", span.origin(), line_no, col_no)
    }
}

#[test]
fn sub_span_iterator() {
    let mut source = "\
one
two
three
four
five
six";
    let span = ("example", (0usize, source.len()));
    let line = source.first_line_of(span);
    assert_eq!(
        line,
        Some(SpannedLine {
            line_num: 1,
            char_count: 3,
            span: ("example", (0, 3)),
        })
    );
    let line = source.next_line_of(span, line.unwrap());
    assert_eq!(
        line,
        Some(SpannedLine {
            line_num: 2,
            char_count: 3,
            span: ("example", (4, 7)),
        })
    );
    let line = source.next_line_of(span, line.unwrap());
    assert_eq!(
        line,
        Some(SpannedLine {
            line_num: 3,
            char_count: 5,
            span: ("example", (8, 13)),
        })
    );
    let line = source.next_line_of(span, line.unwrap());
    assert_eq!(
        line,
        Some(SpannedLine {
            line_num: 4,
            char_count: 4,
            span: ("example", (14, 18)),
        })
    );
    let line = source.next_line_of(span, line.unwrap());
    assert_eq!(
        line,
        Some(SpannedLine {
            line_num: 5,
            char_count: 4,
            span: ("example", (19, 23)),
        })
    );
    let line = source.next_line_of(span, line.unwrap());
    assert_eq!(
        line,
        Some(SpannedLine {
            line_num: 6,
            char_count: 3,
            span: ("example", (24, 27)),
        })
    );
    let line = source.next_line_of(span, line.unwrap());
    assert_eq!(line, None);
}
