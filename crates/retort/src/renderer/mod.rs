use {
    crate::{diagnostic::Diagnostic, style::Stylesheet, Span, SpanResolver},
    std::io,
    termcolor::WriteColor,
};

mod line;
mod list;

pub fn render<'a, Sp: Span>(
    w: &mut dyn WriteColor,
    stylesheet: &mut dyn Stylesheet,
    span_resolver: &mut dyn SpanResolver<Sp>,
    diagnostic: &'a Diagnostic<'a, Sp>,
) -> io::Result<()> {
    let mut w = scopeguard::guard(w, |w| drop(w.reset()));
    let mut list = list::List::new(diagnostic, span_resolver);
    list.write(&mut *w, stylesheet)
}
