use crate::diagnostic::Annotation;
use {
    crate::{
        diagnostic::Diagnostic,
        renderer::line::{Line, RawLine, SourceLine},
        style::{Mark, Stylesheet},
        Span, SpanResolver,
    },
    std::{borrow::Borrow, cmp, fmt, io},
    termcolor::WriteColor,
};

#[derive(Debug)]
pub(super) struct List<'a, 'b, Sp: Span, R: SpanResolver<Sp>> {
    body: Vec<Line<'a, Sp>>,
    span_resolver: &'b mut R,
}

impl<Sp: Span, R: SpanResolver<Sp>> List<'_, '_, Sp, R> {
    pub(super) fn write(
        &mut self,
        w: &mut impl WriteColor,
        style: &mut impl Stylesheet,
    ) -> io::Result<()> {
        let line_num_width = self.body.iter().fold(1, |max, line| match line {
            Line::Source {
                line_num: Some(line_num),
                ..
            } => cmp::max(log10(*line_num), max),
            _ => max,
        });
        for line in &self.body {
            line.write(w, style, self.span_resolver, line_num_width)?;
        }
        Ok(())
    }
}

// FIXME: Calculating `line_num_width`/`marks_width` AOT might eliminate this collection step
impl<'a, 'b, Sp: Span, R: SpanResolver<Sp>> List<'a, 'b, Sp, R> {
    pub(super) fn new(diagnostic: &'a Diagnostic<'a, Sp>, span_resolver: &'b mut R) -> Self {
        let mut body = Vec::new();

        let primary_span = diagnostic.primary.span;
        let origin = diagnostic.primary.span.origin();

        for ann in diagnostic.secondary.iter() {
            if origin != ann.span.origin() {
                // FIXME: implement
                unimplemented!("rendering secondary spans not from origin of primary")
            }
            if ann.span.start() < primary_span.start() || primary_span.end() < ann.span.end() {
                // FIXME: implement
                unimplemented!("rendering secondary spans not a subset of primary")
            }
        }

        body.push(Line::Raw {
            line: RawLine::Title {
                annotation: &diagnostic.primary,
                code: diagnostic.code.as_ref().map(|s| &**s),
            },
        });

        body.push(Line::Raw {
            line: RawLine::Origin(primary_span),
        });

        body.push(Line::Source {
            line_num: None,
            marks: vec![],
            line: SourceLine::Nothing,
        });

        let mut annotations = Vec::with_capacity(diagnostic.secondary.len());
        for ann in diagnostic.secondary.iter() {
            annotations.push(ann.borrow());
        }

        for (line_num, line) in span_resolver.lines_of(primary_span) {
            let line_length = unimplemented!();
            unimplemented!();
        }

        List {
            body,
            span_resolver,
        }
    }
}

fn log10(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        n /= 10;
        sum += 1;
    }
    sum
}
