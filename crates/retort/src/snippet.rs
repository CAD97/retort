use crate::{Annotation, DebugAndDisplay, Message, Slice};

#[derive(Debug, Copy, Clone)]
pub enum Snippet<'a, Sp> {
    Title {
        message: Message<'a>,
        code: Option<&'a str>,
    },
    AnnotatedSlice {
        slice: Slice<'a, Sp>,
        annotations: &'a [Annotation<'a, Sp>],
    },
    Note {
        message: Message<'a>,
    },
    #[doc(hidden)]
    __NonExhaustive,
}
