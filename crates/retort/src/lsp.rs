use {
    crate::diagnostic::*,
    lsp_types::{
        Diagnostic as LSPDiagnostic, DiagnosticRelatedInformation, DiagnosticSeverity,
        NumberOrString, PublishDiagnosticsParams, Url,
    },
    std::collections::HashMap,
};

impl Level {
    fn as_lsp(self) -> DiagnosticSeverity {
        match self {
            Level::Err => DiagnosticSeverity::Error,
            Level::Warn => DiagnosticSeverity::Warning,
            Level::Info => DiagnosticSeverity::Information,
            Level::Hint => DiagnosticSeverity::Hint,
        }
    }
}

pub fn render<Span>(
    diagnostics: impl IntoIterator<Item = Diagnostic<Span>>,
    source: Option<String>,
    mut span_resolver: impl FnMut(Span) -> lsp_types::Location,
) -> Vec<PublishDiagnosticsParams> {
    let mut out: HashMap<Url, Vec<LSPDiagnostic>> = HashMap::new();

    for diagnostic in diagnostics {
        let location = span_resolver(diagnostic.primary.span);
        out.entry(location.uri).or_default().push(LSPDiagnostic {
            range: location.range,
            severity: diagnostic.level.map(Level::as_lsp),
            code: diagnostic.code.map(NumberOrString::String),
            source: source.clone(),
            message: diagnostic.primary.message,
            related_information: Some(
                diagnostic
                    .secondary
                    .into_iter()
                    .map(|diagnostic| DiagnosticRelatedInformation {
                        location: span_resolver(diagnostic.span),
                        message: diagnostic.message,
                    })
                    .collect(),
            ),
        })
    }

    out.into_iter()
        .map(|(uri, diagnostics)| PublishDiagnosticsParams { uri, diagnostics })
        .collect()
}
