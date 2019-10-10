mod builders;
pub use builders::*;

// Note on LSP conversion:
//   range: primary.span.range
//   severity: level
//   code: code
//   source: (give to the renderer)
//   message: primary.message
//   related_information: secondary
//     location.uri: span.origin
//     location.range: span.range
//     message: message

#[derive(Debug, Clone)]
pub struct Diagnostic<Span> {
    pub primary: Annotation<Span>,
    pub code: Option<String>,
    pub secondary: Vec<Annotation<Span>>,
    pub level: Option<Level>,
    non_exhaustive: (),
}

#[derive(Debug, Clone)]
pub struct Annotation<Span> {
    pub span: Span,
    pub message: String,
    non_exhaustive: (),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Level {
    Err,
    Warn,
    Info,
    Hint,
}
