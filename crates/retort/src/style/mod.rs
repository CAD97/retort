use {crate::diagnostic::Level, std::io, termcolor::WriteColor};

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

pub struct NoStyle;

impl Stylesheet for NoStyle {
    fn set_style(&mut self, _w: &mut dyn WriteColor, _style: Style) -> io::Result<()> {
        Ok(())
    }
    fn write_marks(&mut self, w: &mut dyn WriteColor, marks: &[Mark]) -> io::Result<()> {
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
        write!(w, " | ")
    }
    fn write_underline(
        &mut self,
        w: &mut dyn WriteColor,
        _level: Level,
        len: usize,
    ) -> io::Result<()> {
        for _ in 0..len {
            write!(w, "-")?;
        }
        Ok(())
    }
}
