use {
    crate::{
        style::{Style, Stylesheet},
        Level, Snippet, Span, SpanResolver,
    },
    std::io,
    termcolor::WriteColor,
};

mod line;
mod list;

pub fn render<'a, Sp: Span>(
    w: &mut impl WriteColor,
    snippets: &'a [Snippet<'a, Sp>],
    style: &'a mut impl Stylesheet,
    resolver: &'a mut impl SpanResolver<Sp>,
) -> io::Result<()> {
    let mut w = scopeguard::guard(w, |w| drop(w.reset()));
    style.set_style(&mut *w, Style::Base)?;
    list::AnnotatedLines::new(snippets, resolver).write(&mut *w, style, resolver)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mark {
    pub kind: MarkKind,
    pub level: Level,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MarkKind {
    Horizontal,
    Vertical,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    VerticalRight,
    VerticalLeft,
    HorizontalDown,
    HorizontalUp,
    VerticalHorizontal,
    DoubleHorizontal,
    DoubleVertical,
    DoubleDownRight,
    DoubleDownLeft,
    DoubleUpRight,
    DoubleUpLeft,
    DoubleVerticalRight,
    DoubleVerticalLeft,
    DoubleHorizontalDown,
    DoubleHorizontalUp,
    DoubleVerticalHorizontal,
}

impl MarkKind {
    /// Render this mark as an ASCII character.
    /// Note that this reduces marks to singular versions.
    #[rustfmt::skip]
    pub fn as_ascii(self) -> char {
        match self {
            | MarkKind::Horizontal | MarkKind::DoubleHorizontal => '-',
            | MarkKind::Vertical | MarkKind::DoubleVertical => '|',
            | MarkKind::DownRight | MarkKind::DoubleDownRight
            | MarkKind::UpLeft | MarkKind::DoubleUpLeft => '\\',
            | MarkKind::DownLeft | MarkKind::DoubleDownLeft
            | MarkKind::UpRight | MarkKind::DoubleUpRight => '/',
            | MarkKind::VerticalRight | MarkKind::DoubleVerticalRight
            | MarkKind::VerticalLeft | MarkKind::DoubleVerticalLeft
            | MarkKind::HorizontalDown | MarkKind::DoubleHorizontalDown
            | MarkKind::HorizontalUp | MarkKind::DoubleHorizontalUp
            | MarkKind::VerticalHorizontal | MarkKind::DoubleVerticalHorizontal => '+',
        }
    }

    /// Render this mark using Unicode box-drawing characters.
    pub fn as_box_drawing(self) -> char {
        match self {
            MarkKind::Horizontal => '─',
            MarkKind::Vertical => '│',
            MarkKind::DownRight => '┌',
            MarkKind::DownLeft => '┐',
            MarkKind::UpRight => '└',
            MarkKind::UpLeft => '┘',
            MarkKind::VerticalRight => '├',
            MarkKind::VerticalLeft => '┤',
            MarkKind::HorizontalDown => '┬',
            MarkKind::HorizontalUp => '┴',
            MarkKind::VerticalHorizontal => '┼',
            MarkKind::DoubleHorizontal => '═',
            MarkKind::DoubleVertical => '║',
            MarkKind::DoubleDownRight => '╔',
            MarkKind::DoubleDownLeft => '╗',
            MarkKind::DoubleUpRight => '╚',
            MarkKind::DoubleUpLeft => '╝',
            MarkKind::DoubleVerticalRight => '╠',
            MarkKind::DoubleVerticalLeft => '╣',
            MarkKind::DoubleHorizontalDown => '╦',
            MarkKind::DoubleHorizontalUp => '╩',
            MarkKind::DoubleVerticalHorizontal => '╬',
        }
    }
}
