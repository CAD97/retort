use {
    crate::{diagnostic::Diagnostic, Span},
    lsp_types as lsp,
};

pub fn render<'a, Sp: Span + 'a>(
    _diagnostics: impl IntoIterator<Item = Diagnostic<'a, Sp>>,
    _source: Option<&'_ str>,
    _span_resolver: impl FnMut(Sp) -> lsp::Location,
) -> Vec<lsp::PublishDiagnosticsParams> {
    unimplemented!()
}
