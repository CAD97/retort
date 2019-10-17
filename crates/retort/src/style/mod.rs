use {
    crate::{Level, Mark},
    std::io,
    termcolor::WriteColor,
};

mod rustc;
pub use rustc::Rustc;

pub trait Stylesheet {
    /// Set the appropriate style on the sink.
    fn set_style(&mut self, w: &mut impl WriteColor, style: Style) -> io::Result<()>;

    /// Write the series of marks to the sink.
    ///
    /// Each mark should take up one column. The `WriteColor` style on call is
    /// unspecified, and the style may be left in any state at the end of this call.
    fn write_marks(&mut self, w: &mut impl WriteColor, marks: &[Mark]) -> io::Result<()>;

    /// Write an indicator symbol for notes not attached to any span in the snippet.
    /// This symbol is aligned with the separator mark and should be one column wide.
    fn write_note_indicator(&mut self, w: &mut impl WriteColor, level: Level) -> io::Result<()>;

    /// Write an underline of appropriate assertiveness to the sink.
    ///
    /// The underline should span `len` columns. The `WriteColor` style on call is
    /// unspecified, and the style may be left in any state at the end of this call.
    fn underline(&mut self, w: &mut impl WriteColor, level: Level, len: usize) -> io::Result<()>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Style {
    /// No special styling. Typically just resets the writer style.
    Base,
    /// Styling for a message with given severity.
    Level(Level),
    /// Styling for the title line of a snippet.
    TitleLine,
    /// Styling for line numbers.
    LineNumber,
    /// Styling for the origin indicator (e.g. `-->`).
    OriginIndicator,
    /// Styling for the origin path.
    Origin,
    #[doc(hidden)]
    NonExhaustive,
}
