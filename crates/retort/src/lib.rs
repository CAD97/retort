use std::{
    borrow::{Borrow, ToOwned},
    fmt, io,
};
pub use termcolor::WriteColor;

pub mod diagnostic;
//pub mod lsp;
pub mod renderer;
pub mod style;

pub trait Span: fmt::Debug + Copy {
    // ideally, just use Self::OriginHandle::Owned instead
    type OwnedOriginHandle: fmt::Debug + Clone + Borrow<Self::OriginHandle>;
    type OriginHandle: ?Sized + fmt::Debug + ToOwned<Owned = Self::OwnedOriginHandle> + Eq;

    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn new(&self, start: usize, end: usize) -> Self;
    fn origin(&self) -> &Self::OriginHandle;
}

pub trait SpanResolver<Sp> {
    type LineIterator: Iterator<Item = (usize, Sp)> + ExactSizeIterator;

    // ideally, -> impl Iterator<Item=(usize, Sp)> + ExactSizeIterator + '_
    fn lines_of(&mut self, span: Sp) -> Self::LineIterator;
    fn write_span(&mut self, w: &mut dyn WriteColor, span: Sp) -> io::Result<()>;
    fn write_origin(&mut self, w: &mut dyn io::Write, span: Sp) -> io::Result<()>;
}

impl Span for (usize, usize) {
    type OwnedOriginHandle = ();
    type OriginHandle = ();

    fn start(&self) -> usize {
        self.0
    }
    fn end(&self) -> usize {
        self.1
    }
    fn new(&self, start: usize, end: usize) -> Self {
        (start, end)
    }
    fn origin(&self) -> &Self::OriginHandle {
        &()
    }
}

impl<Sp: Span<OriginHandle = (), OwnedOriginHandle = ()>> Span for (&'_ str, Sp) {
    type OwnedOriginHandle = String;
    type OriginHandle = str;

    fn start(&self) -> usize {
        self.1.start()
    }
    fn end(&self) -> usize {
        self.1.end()
    }
    fn new(&self, start: usize, end: usize) -> Self {
        (self.0, self.1.new(start, end))
    }
    fn origin(&self) -> &Self::OriginHandle {
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

impl<'a, Sp: Span> SpanResolver<Sp> for &'a str
where
    Sp::OriginHandle: fmt::Display,
{
    type LineIterator = hidden::SubSpanIterator<'a, Sp>;

    fn lines_of(&mut self, span: Sp) -> Self::LineIterator {
        let start = self[..span.start()].rfind('\n').map_or(0, |i| i + 1);
        let end = self[span.end()..].find('\n').map_or(0, |i| i + 1) + span.end();
        #[allow(clippy::naive_bytecount)] // proof of concept
        hidden::SubSpanIterator {
            span: span.new(start, end),
            text: &self[start..end],
            line_num: bytecount::count(self[..span.start()].as_bytes(), b'\n') + 1, // one indexed
            remaining_lines: bytecount::count(self[start..end].as_bytes(), b'\n')
                + if self.ends_with('\n') { 0 } else { 1 },
        }
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
    let span = (source, (0usize, source.len()));
    let mut lines = source.lines_of(span);
    assert_eq!(lines.len(), 6);
    assert_eq!(lines.next(), Some((1, (source, (0, 3)))));
    assert_eq!(lines.next(), Some((2, (source, (4, 7)))));
    assert_eq!(lines.next(), Some((3, (source, (8, 13)))));
    assert_eq!(lines.next(), Some((4, (source, (14, 18)))));
    assert_eq!(lines.next(), Some((5, (source, (19, 23)))));
    assert_eq!(lines.next(), Some((6, (source, (24, 27)))));
    assert_eq!(lines.next(), None);
}
