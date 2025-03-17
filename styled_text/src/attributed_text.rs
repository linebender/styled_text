// Copyright 2025 the Styled Text Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::vec::Vec;
use core::fmt::Debug;
use core::ops::{Bound, RangeBounds};

use crate::TextStorage;

/// The errors that might happen as a result of [applying] an attribute.
///
/// [applying]: AttributedText::apply_attribute
///
/// TODO: impl Error for this.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ApplyAttributeError {
    /// The bounds given were invalid.
    ///
    /// TODO: Store some data about this here.
    InvalidBounds,
}

/// An attribute and the bounds of the range to which it has been applied.
#[derive(Debug)]
struct RangedAttribute<Attr: Debug> {
    start: Bound<usize>,
    end: Bound<usize>,
    attribute: Attr,
}

/// A block of text with attributes applied to ranges within the text.
#[derive(Debug)]
pub struct AttributedText<T: Debug + TextStorage, Attr: Debug> {
    text: T,
    attributes: Vec<RangedAttribute<Attr>>,
}

impl<T: Debug + TextStorage, Attr: Debug> AttributedText<T, Attr> {
    /// Create an `AttributedText` with no attributes applied.
    pub fn new(text: T) -> Self {
        Self {
            text,
            attributes: Vec::default(),
        }
    }

    /// Apply an `attribute` to a `range` within the text.
    pub fn apply_attribute<R>(
        &mut self,
        range: R,
        attribute: Attr,
    ) -> Result<(), ApplyAttributeError>
    where
        R: RangeBounds<usize>,
    {
        let rend = match range.end_bound() {
            Bound::Included(end) => end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.text.len(),
        };
        if rend > self.text.len() {
            return Err(ApplyAttributeError::InvalidBounds);
        }
        self.attributes.push(RangedAttribute {
            start: range.start_bound().cloned(),
            end: range.end_bound().cloned(),
            attribute,
        });
        Ok(())
    }

    /// Get an iterator over the attributes that apply at the given `index`.
    ///
    /// This doesn't handle conflicting attributes, it just reports everything.
    ///
    /// TODO: Decide if this should also return the range bounds, and if so,
    /// should it return them as `Bound` or as the resolved `usize` values.
    pub fn attributes_at(&self, index: usize) -> impl Iterator<Item = &Attr> {
        self.attributes.iter().filter_map(move |ra| {
            if (ra.start, ra.end).contains(&index) {
                Some(&ra.attribute)
            } else {
                None
            }
        })
    }

    /// Get an iterator over the attributes that apply to the given `range`.
    ///
    /// This doesn't handle conflicting attributes, it just reports everything.
    ///
    /// TODO: Decide if this should also return the range bounds, and if so,
    /// should it return them as `Bound` or as the resolved `usize` values.
    pub fn attributes_for_range<R>(&self, range: R) -> impl Iterator<Item = &Attr>
    where
        R: RangeBounds<usize>,
    {
        fn bounds_to_indices(
            start_bound: Bound<usize>,
            end_bound: Bound<usize>,
            container_length: usize,
        ) -> (usize, usize) {
            let start = match start_bound {
                Bound::Included(start) => start,
                Bound::Excluded(start) => start + 1,
                Bound::Unbounded => 0,
            };
            let end = match end_bound {
                Bound::Included(end) => end + 1,
                Bound::Excluded(end) => end,
                Bound::Unbounded => container_length,
            };
            (start, end)
        }

        let (range_start, range_end) = bounds_to_indices(
            range.start_bound().cloned(),
            range.end_bound().cloned(),
            self.text.len(),
        );

        self.attributes.iter().filter_map(move |ra| {
            let (attribute_start, attribute_end) =
                bounds_to_indices(ra.start, ra.end, self.text.len());

            if (attribute_start < range_end) && (attribute_end > range_start) {
                Some(&ra.attribute)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ApplyAttributeError, AttributedText};

    #[derive(Debug)]
    enum TestAttribute {
        A,
    }

    #[test]
    fn bad_range_for_apply_attribute() {
        let t = "Hello!";
        let mut at = AttributedText::new(t);

        assert_eq!(at.apply_attribute(0..3, TestAttribute::A), Ok(()));
        assert_eq!(at.apply_attribute(0..6, TestAttribute::A), Ok(()));
        assert_eq!(
            at.apply_attribute(0..7, TestAttribute::A),
            Err(ApplyAttributeError::InvalidBounds)
        );
    }
}
