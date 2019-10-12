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
