use {
    crate::{
        diagnostic::{Annotation, Level},
        style::{Mark, Style, Stylesheet},
        Span, SpanResolver,
    },
    std::{fmt, io},
    termcolor::WriteColor,
};

#[derive(Debug, Clone)]
pub(super) enum Line<'a, Sp: Span> {
    Source {
        line_num: Option<usize>,
        marks: Vec<Mark>,
        line: SourceLine<'a, Sp>,
    },
    Raw {
        line: RawLine<'a, Sp>,
    },
}

impl<Sp: Span> Line<'_, Sp> {
    pub(super) fn write(
        &self,
        w: &mut dyn WriteColor,
        style: &mut dyn Stylesheet,
        span_resolver: &mut dyn SpanResolver<Sp>,
        line_num_width: usize,
    ) -> io::Result<()> {
        match self {
            Line::Source {
                line_num,
                marks,
                line,
            } => {
                style.set_style(w, Style::LineNum)?;
                if let Some(line_num) = line_num {
                    write!(w, "{:>1$}", line_num, line_num_width)?;
                } else {
                    write!(w, "{:>1$}", "", line_num_width)?;
                }
                style.write_divider(w)?;
                style.write_marks(w, marks)?;
                line.write(w, style, span_resolver)?;
                style.set_style(w, Style::Base)?;
                writeln!(w)?;
            }
            Line::Raw { line } => line.write(w, style, span_resolver, line_num_width)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) enum SourceLine<'a, Sp: Span> {
    Content(Sp),
    Annotation {
        message: &'a str,
        level: Level,
        underline: (usize, usize),
    },
    Nothing,
}

impl<Sp: Span> SourceLine<'_, Sp> {
    pub(super) fn write(
        &self,
        w: &mut dyn WriteColor,
        style: &mut dyn Stylesheet,
        span_resolver: &mut dyn SpanResolver<Sp>,
    ) -> io::Result<()> {
        match self {
            SourceLine::Content(span) => {
                style.set_style(w, Style::Base)?;
                write!(w, " ")?;
                style.set_style(w, Style::Code)?;
                span_resolver.write_span(w, *span)?;
            }
            SourceLine::Annotation {
                message,
                level,
                underline,
            } => {
                style.set_style(w, Style::Base)?;
                write!(w, "{:>1$}", "", underline.0)?;
                style.write_underline(w, *level, underline.1 - underline.0)?;
                style.set_style(w, Style::Base)?;
                write!(w, " ")?;
                style.set_style(w, Style::Diagnostic(*level))?;
                write!(w, "{}", message)?;
            }
            SourceLine::Nothing => (),
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) enum RawLine<'a, Sp: Span> {
    Origin(Sp),
    Title {
        annotation: &'a Annotation<'a, Sp>,
        code: Option<&'a str>,
    },
}

impl<Sp: Span> RawLine<'_, Sp> {
    pub(super) fn write(
        &self,
        w: &mut dyn WriteColor,
        style: &mut dyn Stylesheet,
        span_resolver: &mut dyn SpanResolver<Sp>,
        line_num_width: usize,
    ) -> io::Result<()> {
        struct EraseColor<'a>(&'a mut dyn WriteColor);
        impl io::Write for EraseColor<'_> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.write(buf)
            }
            fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
                self.0.write_vectored(bufs)
            }
            fn flush(&mut self) -> io::Result<()> {
                self.0.flush()
            }
            fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
                self.0.write_all(buf)
            }
            fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
                self.0.write_fmt(fmt)
            }
            fn by_ref(&mut self) -> &mut Self
            where
                Self: Sized,
            {
                self
            }
        }

        match self {
            RawLine::Origin(span) => {
                style.set_style(w, Style::OriginLine)?;
                write!(w, "{:>1$}", "", line_num_width)?;
                span_resolver.write_origin(&mut EraseColor(w), *span)?;
                style.set_style(w, Style::Base)?;
                writeln!(w)?;
            }
            RawLine::Title { annotation, code } => {
                style.set_style(w, Style::TitleLine)?;
                write!(w, "{}", annotation.level)?;
                if let Some(code) = code {
                    write!(w, "[{}]", code)?;
                }
                write!(w, ": ")?;
                style.set_style(w, Style::Diagnostic(annotation.level))?;
                write!(w, "{}", annotation.message)?;
                style.set_style(w, Style::Base)?;
                writeln!(w)?;
            }
        }
        Ok(())
    }
}
