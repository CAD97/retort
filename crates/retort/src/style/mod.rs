#[cfg(feature = "syntect")]
use syntect::highlighting::Style as SyntectStyle;
use {
    crate::{Level, Mark},
    std::io,
    termcolor::WriteColor,
};

mod highlighting;
#[cfg(feature = "syntect")]
pub use highlighting::apply_syntect_style;
pub use highlighting::SyntaxHighlighter;

pub trait Stylesheet {
    /// Get the syntect [`Highlighter`], if there is one.
    /// This should not return `Some` unless the syntect feature is enabled.
    /// This might return `None` even with the syntect feature enabled,
    /// if the stylesheet does not have support for syntect highlighting.
    ///
    ///   [`Highlighter`]: <https://docs.rs/syntect/3/syntect/highlighting/struct.Highlighter.html>
    fn highlighter(&self) -> Option<SyntaxHighlighter<'_>>;

    /// Set the appropriate style on the sink.
    fn set_style(&mut self, w: &mut dyn WriteColor, style: Style) -> io::Result<()>;

    /// Write the series of marks to the sink.
    ///
    /// Each mark should take up one column. The `WriteColor` style on call is
    /// unspecified, and the style may be left in any state at the end of this call.
    fn write_marks(&mut self, w: &mut dyn WriteColor, marks: &[Mark]) -> io::Result<()>;

    /// Write an indicator symbol for notes not attached to any span in the snippet.
    /// This symbol is aligned with the separator mark and should be one column wide.
    fn write_note_indicator(&mut self, w: &mut dyn WriteColor, level: Level) -> io::Result<()>;

    /// Write an underline of appropriate assertiveness to the sink.
    ///
    /// The underline should span `len` columns. The `WriteColor` style on call is
    /// unspecified, and the style may be left in any state at the end of this call.
    fn underline(&mut self, w: &mut dyn WriteColor, level: Level, len: usize) -> io::Result<()>;
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
    /// Styling for code, using syntect themes.
    #[cfg(feature = "syntect")]
    #[cfg_attr(doc, doc(cfg(feature = "syntect")))]
    Syntect(SyntectStyle),
    #[doc(hidden)]
    NonExhaustive,
}
