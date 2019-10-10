use super::*;
use std::{fmt::Display, mem};

impl<Span: crate::Span> Diagnostic<Span> {
    pub fn build() -> DiagnosticBuilder<Span> {
        DiagnosticBuilder {
            primary: None,
            code: None,
            secondary: vec![],
            level: None,
        }
    }
}

#[derive(Debug)]
pub struct DiagnosticBuilder<Span> {
    primary: Option<Annotation<Span>>,
    code: Option<String>,
    secondary: Vec<Annotation<Span>>,
    level: Option<Level>,
}

impl<Span: crate::Span> DiagnosticBuilder<Span> {
    pub fn primary(&mut self, annotation: Annotation<Span>) -> &mut Self {
        self.primary = Some(annotation);
        self
    }

    pub fn code(&mut self, code: impl Display) -> &mut Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn secondary(&mut self, annotation: Annotation<Span>) -> &mut Self {
        self.secondary.push(annotation);
        self
    }

    pub fn level(&mut self, level: Level) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn build(&mut self) -> Diagnostic<Span> {
        let mut swindle = vec![];
        mem::swap(&mut swindle, &mut self.secondary);
        Diagnostic {
            primary: self
                .primary
                .take()
                .expect("incomplete `Diagnostic` without `primary`"),
            code: self.code.take(),
            secondary: swindle,
            level: self.level.take(),
            non_exhaustive: (),
        }
    }
}

impl<Span: crate::Span> Annotation<Span> {
    pub fn build() -> AnnotationBuilder<Span> {
        AnnotationBuilder {
            span: None,
            message: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct AnnotationBuilder<Span> {
    span: Option<Span>,
    message: String,
}

impl<Span: crate::Span> AnnotationBuilder<Span> {
    pub fn span(&mut self, span: impl Into<Span>) -> &mut Self {
        self.span = Some(span.into());
        self
    }

    pub fn message(&mut self, message: impl Display) -> &mut Self {
        self.message = message.to_string();
        self
    }

    pub fn build(&mut self) -> Annotation<Span> {
        let mut swizzle = String::new();
        mem::swap(&mut swizzle, &mut self.message);
        Annotation {
            span: self
                .span
                .take()
                .expect("incomplete `Annotation` without `span`"),
            message: swizzle,
            non_exhaustive: (),
        }
    }
}
