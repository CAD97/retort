use {
    crate::Span,
    std::{
        borrow::{Borrow, Cow},
        fmt, mem,
    },
};

#[derive(Debug, Clone)]
pub struct Diagnostic<'a, Sp: Span> {
    pub primary: Annotation<'a, Sp>,
    pub code: Option<Cow<'a, str>>,
    pub secondary: Cow<'a, [Annotation<'a, Sp>]>,
}

#[derive(Debug, Clone)]
pub struct Annotation<'a, Sp: Span> {
    pub span: Sp,
    pub level: Level,
    pub message: Cow<'a, str>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Level {
    Err,
    Warn,
    Info,
    Hint,
}

impl<Sp: Span> Diagnostic<'_, Sp> {
    pub fn borrow(&self) -> Diagnostic<'_, Sp> {
        Diagnostic {
            primary: self.primary.borrow(),
            code: self.code.as_ref().map(|code| Cow::Borrowed(code.borrow())),
            secondary: Cow::Borrowed(self.secondary.borrow()),
        }
    }

    pub fn into_owned(self) -> Diagnostic<'static, Sp> {
        let mut secondary: Vec<Annotation<'_, Sp>> = self.secondary.into_owned();
        // Scratch space:
        let mut temp = Annotation {
            span: self.primary.span,
            level: Level::Err,
            message: "".into(),
        };
        // Convert every annotation to the owned form:
        for ann in &mut secondary {
            // We have to do the conversion in scratch space since it's by-value,
            // so we put a dummy annotation in place temporarily.
            // The scratch space can be eliminated with more `unsafe`,
            // but I can't find the `fn(&mut T, impl FnOnce(T) -> T)` crate.
            // https://www.reddit.com/r/rust/comments/deeoph/_/f3dmoot
            mem::swap(&mut temp, ann);
            temp = temp.into_owned();
            mem::swap(&mut temp, ann);
        }
        // Now that every annotation is owned, we can erase the lifetime soundly.
        let secondary: Vec<Annotation<'static, Sp>> = unsafe { mem::transmute(secondary) };
        Diagnostic {
            primary: self.primary.into_owned(),
            code: self.code.map(|code| Cow::Owned(code.into_owned())),
            secondary: Cow::Owned(secondary),
        }
    }
}

impl<Sp: crate::Span> Annotation<'_, Sp> {
    pub fn borrow(&self) -> Annotation<'_, Sp> {
        Annotation {
            span: self.span,
            level: self.level,
            message: Cow::Borrowed(self.message.borrow()),
        }
    }

    pub fn into_owned(self) -> Annotation<'static, Sp> {
        Annotation {
            span: self.span,
            level: self.level,
            message: Cow::Owned(self.message.into_owned()),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Err => f.write_str("error"),
            Level::Warn => f.write_str("warning"),
            Level::Info => f.write_str("info"),
            Level::Hint => f.write_str("note"),
        }
    }
}
