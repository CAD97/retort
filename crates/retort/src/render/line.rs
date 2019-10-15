use crate::render::MarkKind;
use {
    crate::{
        style::{Style, Stylesheet},
        DebugAndDisplay, Level, Mark, Span, SpanResolver,
    },
    std::{io, ops::Range},
    termcolor::WriteColor,
};

#[derive(Debug, Clone)]
pub(super) enum Line<'a, Sp> {
    Title {
        text: &'a dyn DebugAndDisplay,
        level: Level,
        code: Option<&'a dyn DebugAndDisplay>,
    },
    Origin {
        file: &'a dyn DebugAndDisplay,
        pos: Option<(usize, Option<usize>)>,
    },
    Source {
        line_num: Option<usize>,
        marks: Vec<Mark>,
        line: SourceLine<'a, Sp>,
    },
    Note {
        text: &'a dyn DebugAndDisplay,
        level: Level,
    },
    #[allow(dead_code)]
    Fold { marks: Vec<Mark> },
}

impl<Sp: Span> Line<'_, Sp> {
    pub(super) fn write(
        &self,
        w: &mut dyn WriteColor,
        style: &mut dyn Stylesheet,
        resolver: &mut dyn SpanResolver<Sp>,
        line_num_width: usize,
        max_marks: usize,
    ) -> io::Result<()> {
        const SEPARATOR: Mark = Mark {
            kind: MarkKind::DoubleVertical,
            level: Level::Information,
        };

        match self {
            Line::Title { text, level, code } => {
                style.set_style(w, Style::Level(*level))?;
                write!(w, "{}", level)?;
                if let Some(code) = code {
                    write!(w, "[{}]", code)?;
                }
                write!(w, ": ")?;
                style.set_style(w, Style::TitleLine)?;
                write!(w, "{}", text)?;
            }
            Line::Origin { file, pos } => {
                style.set_style(w, Style::OriginIndicator)?;
                write!(w, "{:>1$}", "", line_num_width)?;
                write!(w, "--> ")?;
                style.set_style(w, Style::Origin)?;
                write!(w, "{}", file)?;
                if let Some((line, col)) = pos {
                    write!(w, ":{}", line)?;
                    if let Some(col) = col {
                        write!(w, ":{}", col)?;
                    }
                }
            }
            Line::Source {
                line_num,
                marks,
                line,
            } => {
                style.set_style(w, Style::LineNumber)?;
                if let Some(line_num) = line_num {
                    write!(w, "{:>1$} ", line_num, line_num_width)?;
                } else {
                    write!(w, "{:>1$} ", "", line_num_width)?;
                }
                style.write_marks(w, &[SEPARATOR])?;
                write!(w, " {:>1$}", "", max_marks - marks.len())?;
                style.write_marks(w, marks)?;
                write!(w, " ")?;
                line.write(w, style, resolver)?;
            }
            Line::Note { text, level } => {
                write!(w, "{:>1$} ", "", line_num_width)?;
                style.write_note_indicator(w, *level)?;
                style.set_style(w, Style::Level(*level))?;
                write!(w, " {}", text)?;
            }
            Line::Fold { marks } => {
                write!(w, "{:>1$}", "...", line_num_width + 1)?;
                style.write_marks(w, &[SEPARATOR])?;
                write!(w, " ")?;
                style.write_marks(w, marks)?;
            }
        }
        style.set_style(w, Style::Base)?;
        writeln!(w)
    }
}

#[derive(Debug, Clone)]
pub(super) enum SourceLine<'a, Sp> {
    Content {
        span: Sp,
    },
    Annotation {
        text: &'a dyn DebugAndDisplay,
        level: Level,
        underline: Range<usize>,
    },
    Spacing,
}

impl<Sp: Span> SourceLine<'_, Sp> {
    pub(super) fn write(
        &self,
        w: &mut dyn WriteColor,
        style: &mut dyn Stylesheet,
        resolver: &mut dyn SpanResolver<Sp>,
    ) -> io::Result<()> {
        match self {
            SourceLine::Content { span } => {
                style.set_style(w, Style::Base)?;
                resolver.write_span(w, style, span)?;
            }
            SourceLine::Annotation {
                text,
                level,
                underline,
            } => {
                write!(w, "{:>1$}", "", underline.start)?;
                style.underline(w, *level, underline.len())?;
                style.set_style(w, Style::Level(*level))?;
                write!(w, " {}", text)?;
            }
            SourceLine::Spacing => (),
        }
        Ok(())
    }
}
