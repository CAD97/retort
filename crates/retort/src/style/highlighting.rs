#[cfg(feature = "syntect")]
use {
    std::ops::Deref,
    syntect::highlighting::{FontStyle, Highlighter, Style as SyntectStyle, Theme},
    termcolor::{Color, ColorSpec},
};
use {
    std::{io, marker::PhantomData},
    termcolor::WriteColor,
};

/// Source code syntax highlighting style based on [syntect].
///
///   [syntect]: <https://crates.io/crates/syntect>
pub struct SyntaxHighlighter<'a> {
    #[cfg(feature = "syntect")]
    inner: Highlighter<'a>,
    phantom: PhantomData<&'a ()>,
}

#[cfg(feature = "syntect")]
#[cfg_attr(doc, doc(cfg(feature = "syntect")))]
impl<'a> Deref for SyntaxHighlighter<'a> {
    type Target = Highlighter<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(feature = "syntect")]
#[cfg_attr(doc, doc(cfg(feature = "syntect")))]
impl<'a> SyntaxHighlighter<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        SyntaxHighlighter {
            inner: Highlighter::new(theme),
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "syntect")]
#[cfg_attr(doc, doc(cfg(feature = "syntect")))]
pub fn apply_syntect_style(w: &mut dyn WriteColor, style: SyntectStyle) -> io::Result<()> {
    w.set_color(
        &ColorSpec::new()
            .set_fg(Some(Color::Rgb(
                style.foreground.r,
                style.foreground.g,
                style.foreground.b,
            )))
            .set_bg(Some(Color::Rgb(
                style.background.r,
                style.background.g,
                style.background.b,
            )))
            .set_bold(style.font_style.contains(FontStyle::BOLD))
            .set_underline(style.font_style.contains(FontStyle::UNDERLINE)),
    )
}
