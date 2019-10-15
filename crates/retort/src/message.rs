use std::fmt;

/// Trait to create trait objects with both `Debug` and `Display` capability.
pub trait DebugAndDisplay: fmt::Debug + fmt::Display {
    fn as_debug(&mut self) -> &mut dyn fmt::Debug;
    fn as_display(&mut self) -> &mut dyn fmt::Display;
}

#[rustfmt::skip]
impl<D: fmt::Debug + fmt::Display> DebugAndDisplay for D {
    fn as_debug(&mut self) -> &mut dyn fmt::Debug { self }
    fn as_display(&mut self) -> &mut dyn fmt::Display { self }
}

#[derive(Debug, Copy, Clone)]
pub struct Message<'a> {
    pub text: &'a dyn DebugAndDisplay,
    pub level: Level,
}

/// A level of severity for an annotation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Level {
    /// An error on the hand of the user.
    Error,
    /// A warning of something that isn't necessarily wrong, but looks fishy.
    Warning,
    /// An informational annotation.
    Information,
    /// A hint about what actions can be taken.
    Hint,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::Error => f.write_str("error"),
            Level::Warning => f.write_str("warning"),
            Level::Information => f.write_str("info"),
            Level::Hint => f.write_str("hint"),
        }
    }
}
