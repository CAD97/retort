use {
    crate::{
        diagnostic::Diagnostic,
        renderer::line::{Line, RawLine, SourceLine},
        style::{Mark, Stylesheet},
        Span, SpanResolver, SpannedLine,
    },
    std::{borrow::Cow, io},
    termcolor::WriteColor,
};

#[inline(always)]
#[allow(clippy::ptr_arg)]
fn get_borrow<'a, T>(cow: &Cow<'a, T>) -> &'a T
where
    T: 'a + ToOwned + ?Sized,
{
    match cow {
        &Cow::Borrowed(t) => t,
        Cow::Owned(_) => unreachable!(),
    }
}

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
        let line_num_width = self
            .body
            .iter()
            .map(|line| match line {
                Line::Source {
                    line_num: Some(line_num),
                    ..
                } => log10(*line_num),
                _ => 1,
            })
            .max()
            .unwrap_or(1);
        for line in &self.body {
            line.write(w, style, self.span_resolver, line_num_width)?;
        }
        Ok(())
    }
}

// FIXME: Calculating `line_num_width`/`marks_width` AOT might eliminate this collection step
impl<'a, 'b, Sp: Span, R: SpanResolver<Sp>> List<'a, 'b, Sp, R> {
    pub(super) fn new(diagnostic: &'a Diagnostic<'a, Sp>, span_resolver: &'b mut R) -> Self {
        let mut body = Vec::<Line<'a, Sp>>::new();

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

        let mut process = |SpannedLine { line_num, span, .. }: SpannedLine<Sp>| {
            let mut annotations_here = vec![];
            let mut marks = vec![];

            annotations.retain(|ann| {
                if span.start() <= ann.span.start() && ann.span.end() <= span.end() {
                    // Annotation in this line
                    annotations_here.push(ann.clone()); // cloning borrowed data
                    false
                } else if span.start() <= ann.span.start() && ann.span.start() <= span.end() {
                    // Annotation starts in this line
                    marks.push(Mark::Start);
                    true
                } else if ann.span.start() < span.start() && span.end() < ann.span.end() {
                    // Annotation goes through this line
                    marks.push(Mark::Continue);
                    true
                } else if ann.span.start() < span.start() && ann.span.end() <= span.end() {
                    // Annotation ends on this line
                    marks.push(Mark::Continue);
                    annotations_here.push(ann.clone()); // cloning borrowed data
                    false
                } else {
                    // Annotation starts on later line
                    true
                }
            });

            body.push(Line::Source {
                line_num: Some(line_num),
                marks,
                line: SourceLine::Content(span),
            });

            for ann in annotations_here {
                // FIXME: this is byte position
                let start = if span.start() < ann.span.start() {
                    ann.span.start() - span.start()
                } else {
                    0
                };
                let marks = if ann.span.start() < span.start() {
                    vec![Mark::End]
                } else {
                    vec![]
                };
                body.push(Line::Source {
                    line_num: None,
                    marks,
                    line: SourceLine::Annotation {
                        message: get_borrow(&ann.message), // cloning borrowed data
                        level: ann.level,
                        underline: (start, ann.span.end() - span.start()),
                    },
                })
            }
        };

        if let Some(mut line) = span_resolver.first_line_of(primary_span) {
            process(line);
            while let Some(next) = span_resolver.next_line_of(primary_span, line) {
                line = next;
                process(line);
            }
        }

        let max_marks = body
            .iter()
            .map(|line| match line {
                Line::Source { marks, .. } => marks.len(),
                Line::Raw { .. } => 0,
            })
            .max()
            .unwrap_or(0);

        for line in &mut body {
            match line {
                Line::Source { marks, .. } => {
                    // FIXME: this is horribly inefficient
                    while marks.len() < max_marks {
                        marks.insert(0, Mark::None);
                    }
                }
                _ => {}
            }
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
