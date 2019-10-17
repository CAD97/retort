use {
    crate::{
        style::{Style, Stylesheet},
        Level, Mark,
    },
    std::io,
    termcolor::{Color, ColorSpec, WriteColor},
};

pub struct Rustc;

impl Stylesheet for Rustc {
    fn set_style(&mut self, w: &mut impl WriteColor, style: Style) -> io::Result<()> {
        match style {
            Style::Level(Level::Error) => w.set_color(ColorSpec::new().set_fg(Some(Color::Red))),
            Style::Level(Level::Warning) => {
                w.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            }
            Style::Level(Level::Information) => {
                w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
            }
            Style::TitleLine => {
                w.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))
            }
            Style::OriginIndicator => w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))),
            Style::LineNumber => w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))),
            _ => w.reset(),
        }
    }

    fn write_marks(&mut self, w: &mut impl WriteColor, marks: &[Mark]) -> io::Result<()> {
        for mark in marks {
            self.set_style(w, Style::Level(mark.level))?;
            write!(w, "{}", mark.kind.as_ascii())?;
        }
        Ok(())
    }

    fn write_note_indicator(&mut self, w: &mut impl WriteColor, level: Level) -> io::Result<()> {
        w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(w, "=")?;
        w.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))?;
        write!(w, " {}:", level)?;
        Ok(())
    }

    fn underline(&mut self, w: &mut impl WriteColor, level: Level, len: usize) -> io::Result<()> {
        self.set_style(w, Style::Level(level))?;
        let ch = match level {
            Level::Error | Level::Warning => '^',
            Level::Information | Level::Hint => '-',
        };
        for _ in 0..len {
            write!(w, "{}", ch)?;
        }
        Ok(())
    }
}
