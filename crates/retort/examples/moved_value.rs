use retort::*;
use std::io;
use termcolor::{ColorChoice, StandardStream};

fn main() -> io::Result<()> {
    let mut source = include_str!("moved_value.txt");
    let snippets = &[
        Snippet::Title {
            message: Message {
                text: &"use of moved value: `x`",
                level: Level::Error,
            },
            code: Some(&"E0382"),
        },
        Snippet::AnnotatedSlice {
            slice: Slice {
                span: 157..158,
                origin: Some(Origin { file: &"examples/moved_value.rs", pos: Some((4, Some(5))) }),
                spacing: Spacing::TightBelow,
                fold: false,
            },
            annotations: &[
                Annotation {
                    span: 157..158,
                    message: Message {
                        text: &"move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait",
                        level: Level::Information,
                    },
                },
            ],
        },
        Snippet::AnnotatedSlice {
            slice: Slice {
                span: 184..185,
                origin: None,
                spacing: Spacing::Tight,
                fold: false,
            },
            annotations: &[
                Annotation {
                    span: 184..185,
                    message: Message {
                        text: &"value moved here",
                        level: Level::Information,
                    },
                },
            ],
        },
        Snippet::AnnotatedSlice {
            slice: Slice {
                span: 192..193,
                origin: None,
                spacing: Spacing::Tight,
                fold: false,
            },
            annotations: &[
                Annotation {
                    span: 192..193,
                    message: Message {
                        text: &"value used here after move",
                        level: Level::Error,
                    },
                }
            ],
        }
    ];
    let out = StandardStream::stdout(ColorChoice::Auto);
    let mut out = out.lock();
    render(&mut out, snippets, &mut style::Rustc, &mut source)
}
