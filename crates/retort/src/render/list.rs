use {
    crate::{
        render::{
            line::{Line, SourceLine},
            Mark, MarkKind,
        },
        style::Stylesheet,
        Annotation, Snippet, Spacing, Span, SpanResolver,
    },
    std::io,
    termcolor::WriteColor,
};

pub(super) struct AnnotatedLines<'a, Sp: Span> {
    body: Vec<Line<'a, Sp>>,
}

// FIXME: This is quick-and-dirty implementation,
//     see if there are ways to improve iteration complexity
impl<'a, Sp: Span> AnnotatedLines<'a, Sp> {
    pub(super) fn new(
        snippets: &'a [Snippet<'a, Sp>],
        resolver: &'_ mut dyn SpanResolver<Sp>,
    ) -> Self {
        let mut body: Vec<Line<'a, Sp>> = Vec::new();

        for snippet in snippets {
            match snippet {
                Snippet::Title { message, code } => {
                    body.push(Line::Title {
                        text: message.text,
                        level: message.level,
                        code: *code,
                    });
                }
                Snippet::Note { message } => {
                    body.push(Line::Note {
                        text: message.text,
                        level: message.level,
                    });
                }
                Snippet::AnnotatedSlice { slice, annotations } => {
                    if let Some(origin) = slice.origin {
                        body.push(Line::Origin {
                            file: origin.file,
                            pos: origin.pos,
                        })
                    }
                    match slice.spacing {
                        Spacing::TightBelow | Spacing::Spacious => {
                            body.push(Line::Source {
                                line_num: None,
                                marks: vec![],
                                line: SourceLine::Spacing,
                            });
                        }
                        _ => (),
                    }
                    let mut remaining_annotations = annotations.iter().collect::<Vec<_>>();

                    // FIXME: eliminate macro-hidden repetition here
                    macro_rules! process {
                        ($line:expr) => {{
                            let line = $line;
                            let mut annotations_here: Vec<&Annotation<'_, Sp>> = vec![];
                            let mut marks = vec![];
                            remaining_annotations.retain(|ann| {
                                if line.span.start() <= ann.span.start()
                                    && ann.span.end() <= line.span.end()
                                {
                                    // Annotation in this line
                                    annotations_here.push(ann);
                                    false
                                } else if line.span.start() <= ann.span.start()
                                    && ann.span.start() <= line.span.end()
                                {
                                    // Annotation starts in this line
                                    marks.push(Mark {
                                        kind: MarkKind::DownRight,
                                        level: ann.message.level,
                                    });
                                    true
                                } else if ann.span.start() < line.span.start()
                                    && line.span.end() < ann.span.end()
                                {
                                    // Annotation goes through this line
                                    marks.push(Mark {
                                        kind: MarkKind::Vertical,
                                        level: ann.message.level,
                                    });
                                    true
                                } else if ann.span.start() < line.span.start()
                                    && ann.span.end() <= line.span.end()
                                {
                                    // Annotation ends on this line
                                    marks.push(Mark {
                                        kind: MarkKind::Vertical,
                                        level: ann.message.level,
                                    });
                                    annotations_here.push(ann);
                                    false
                                } else {
                                    // Annotation starts on later line
                                    true
                                }
                            });
                            body.push(Line::Source {
                                line_num: Some(line.num),
                                marks,
                                line: SourceLine::Content {
                                    span: line.span.clone(),
                                },
                            });
                            for ann in annotations_here {
                                // FIXME: This is blindly copying annotate-snippets@0.6
                                //     Needs to be fixed to separate `Mark` line and underline
                                let start = if line.span.start() < ann.span.start() {
                                    resolver.count_chars(
                                        &line.span.new(line.span.start(), ann.span.start()),
                                    )
                                } else {
                                    0
                                };
                                let marks = if ann.span.start() < line.span.start() {
                                    vec![Mark {
                                        kind: MarkKind::UpRight,
                                        level: ann.message.level,
                                    }]
                                } else {
                                    vec![]
                                };
                                body.push(Line::Source {
                                    line_num: None,
                                    marks,
                                    line: SourceLine::Annotation {
                                        text: ann.message.text,
                                        level: ann.message.level,
                                        underline: start
                                            ..resolver.count_chars(
                                                &line.span.new(line.span.start(), ann.span.end()),
                                            ),
                                    },
                                })
                            }
                        }};
                    }

                    // FIXME: implement folding
                    let mut line = resolver.first_line_of(&slice.span);
                    process!(&line);
                    while let Some(next) = resolver.next_line_of(&slice.span, line) {
                        line = next;
                        process!(&line);
                    }
                    match slice.spacing {
                        Spacing::TightAbove | Spacing::Spacious => {
                            body.push(Line::Source {
                                line_num: None,
                                marks: vec![],
                                line: SourceLine::Spacing,
                            });
                        }
                        _ => (),
                    }
                }
                Snippet::__NonExhaustive => unreachable!(),
            }
        }

        AnnotatedLines { body }
    }
}

impl<Sp: Span> AnnotatedLines<'_, Sp> {
    pub(super) fn write(
        &mut self,
        w: &mut dyn WriteColor,
        style: &mut dyn Stylesheet,
        resolver: &mut dyn SpanResolver<Sp>,
    ) -> io::Result<()> {
        let max_line_no = self
            .body
            .iter()
            .filter_map(|line| match line {
                Line::Fold { .. } => Some(10), // to account for length of `...`
                Line::Source { line_num, .. } => *line_num,
                _ => None,
            })
            .max()
            .unwrap_or(1);
        let line_num_width = log10usize(max_line_no);
        let max_marks = self
            .body
            .iter()
            .filter_map(|line| match line {
                Line::Source { marks, .. } | Line::Fold { marks } => Some(marks.len()),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        for line in &self.body {
            line.write(w, style, resolver, line_num_width, max_marks)?;
        }
        Ok(())
    }
}

fn log10usize(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        n /= 10;
        sum += 1;
    }
    sum
}
