//use {
//    std::{fmt, ops::Range},
//};

pub mod diagnostic;
pub mod lsp;

//trait Span: fmt::Debug + Clone {
//    type SourceHandle: Clone;
//    type Index: Copy;
//
//    fn start(&self) -> Self::Index;
//    fn end(&self) -> Self::Index;
//    fn new(&self, start: Self::Index, end: Self::Index) -> Self;
//    fn resource(&self) -> Option<&Self::SourceHandle>;
//}
//
//impl Span for Range<usize> {
//    type SourceHandle = std::convert::Infallible;
//    type Index = usize;
//
//    fn start(&self) -> usize {
//        self.start
//    }
//
//    fn end(&self) -> usize {
//        self.end
//    }
//
//    fn new(&self, start: usize, end: usize) -> Self {
//        start..end
//    }
//
//    fn resource(&self) -> Option<&Self::SourceHandle> {
//        None
//    }
//}
