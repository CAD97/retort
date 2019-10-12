use {crate::diagnostic::Level, std::io, termcolor::WriteColor};
use termcolor::{ColorSpec, Color};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Mark {
    None,
    Start,
    Continue,
    End,
}

#[derive(Debug, Copy, Clone)]
pub enum Style {
    Base,
    Code,
    Diagnostic(Level),
    LineNum,
    TitleLine,
    OriginLine,
    #[doc(hidden)]
    NonExhaustive,
}

pub trait Stylesheet {
    fn set_style(&mut self, w: &mut dyn WriteColor, style: Style) -> io::Result<()>;
    fn write_marks(&mut self, w: &mut dyn WriteColor, marks: &[Mark]) -> io::Result<()>;
    fn write_divider(&mut self, w: &mut dyn WriteColor) -> io::Result<()>;
    fn write_underline(
        &mut self,
        w: &mut dyn WriteColor,
        level: Level,
        len: usize,
    ) -> io::Result<()>;
}

pub struct TestStyle;

impl Stylesheet for TestStyle {
    fn set_style(&mut self, w: &mut dyn WriteColor, style: Style) -> io::Result<()> {
        match style {
            Style::Diagnostic(Level::Err) => w.set_color(ColorSpec::new().set_fg(Some(Color::Red))),
            Style::Diagnostic(Level::Warn) => w.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))),
            Style::TitleLine => w.set_color(ColorSpec::new().set_bold(true)),
            Style::LineNum => w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))),
            _ => w.reset(),
        }
    }
    fn write_marks(&mut self, w: &mut dyn WriteColor, marks: &[Mark]) -> io::Result<()> {
        w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        for mark in marks {
            match mark {
                Mark::None => write!(w, " ")?,
                Mark::Start => write!(w, "/")?,
                Mark::Continue => write!(w, "|")?,
                Mark::End => write!(w, "\\")?,
            }
        }
        Ok(())
    }
    fn write_divider(&mut self, w: &mut dyn WriteColor) -> io::Result<()> {
        w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(w, " | ")
    }
    fn write_underline(
        &mut self,
        w: &mut dyn WriteColor,
        _level: Level,
        len: usize,
    ) -> io::Result<()> {
        w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        for _ in 0..len {
            write!(w, "-")?;
        }
        Ok(())
    }
}
