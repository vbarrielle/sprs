//! This module abstracts over ranges to allow functions in the crate to take
//! ranges as input in an ergonomic way. It howvers seals this abstractions to
//! leave full control in this crate.

/// Abstract over `std::ops::{Range,RangeFrom,RangeTo,RangeFull}`
pub trait Range {
    fn start(&self) -> Option<usize>;

    fn end(&self) -> Option<usize>;
}

impl Range for std::ops::Range<usize> {
    fn start(&self) -> Option<usize> {
        Some(self.start)
    }

    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl Range for std::ops::RangeFrom<usize> {
    fn start(&self) -> Option<usize> {
        Some(self.start)
    }

    fn end(&self) -> Option<usize> {
        None
    }
}

impl Range for std::ops::RangeTo<usize> {
    fn start(&self) -> Option<usize> {
        None
    }

    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl Range for std::ops::RangeFull {
    fn start(&self) -> Option<usize> {
        None
    }

    fn end(&self) -> Option<usize> {
        None
    }
}