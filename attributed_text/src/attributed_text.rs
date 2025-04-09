// Copyright 2025 the Styled Text Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::vec::Vec;
use core::fmt::Debug;
use core::ops::Range;

use crate::{EditBehavior, EditableTextStorage, SpanEditAction, TextStorage};

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

/// The errors that might happen as a result of [deleting] from an
/// [`AttributedText`].
///
/// [deleting]: AttributedText::delete
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeleteError {
    /// The bounds given were invalid.
    ///
    /// TODO: Store some data about this here.
    InvalidRange,
}

/// A block of text with attributes applied to ranges within the text.
#[derive(Debug)]
pub struct AttributedText<T: Debug + TextStorage, Attr: Debug> {
    text: T,
    attributes: Vec<(Range<usize>, Attr)>,
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
    pub fn apply_attribute(
        &mut self,
        range: Range<usize>,
        attribute: Attr,
    ) -> Result<(), ApplyAttributeError> {
        let text_len = self.text.len();
        if range.start > text_len || range.end > text_len {
            return Err(ApplyAttributeError::InvalidBounds);
        }
        self.attributes.push((range, attribute));
        Ok(())
    }

    /// Get an iterator over the attributes that apply at the given `index`.
    ///
    /// This doesn't handle conflicting attributes, it just reports everything.
    ///
    /// TODO: Decide if this should also return the spans' ranges.
    pub fn attributes_at(&self, index: usize) -> impl Iterator<Item = &Attr> {
        self.attributes.iter().filter_map(move |(attr_span, attr)| {
            if attr_span.contains(&index) {
                Some(attr)
            } else {
                None
            }
        })
    }

    /// Get an iterator over the attributes that apply to the given `range`.
    ///
    /// This doesn't handle conflicting attributes, it just reports everything.
    ///
    /// TODO: Decide if this should also return the spans' ranges.
    pub fn attributes_for_range(&self, range: Range<usize>) -> impl Iterator<Item = &Attr> {
        self.attributes.iter().filter_map(move |(attr_span, attr)| {
            if (attr_span.start < range.end) && (attr_span.end > range.start) {
                Some(attr)
            } else {
                None
            }
        })
    }
}

impl<T: Debug + EditableTextStorage, Attr: Debug + EditBehavior> AttributedText<T, Attr> {
    #[expect(missing_docs, reason = "TODO")]
    pub fn delete(&mut self, deletion_range: Range<usize>) -> Result<(), DeleteError> {
        let text_len = self.text.len();
        if deletion_range.start > deletion_range.end || deletion_range.end > text_len {
            return Err(DeleteError::InvalidRange);
        }

        // Remove the text
        self.text.replace_range(deletion_range.clone(), "");

        let deleted_len = deletion_range.end - deletion_range.start;

        // Adjust attribute spans
        self.attributes.retain_mut(|(span, attr)| {
            if span.end <= deletion_range.start {
                // Completely before delete — no change
                true
            } else if span.start >= deletion_range.end {
                // Completely after delete — shift left
                span.start -= deleted_len;
                span.end -= deleted_len;
                true
            } else {
                match attr.on_edit() {
                    SpanEditAction::Keep => {
                        if span.start < deletion_range.start && span.end > deletion_range.end {
                            // Span fully covers deletion -> shrink gap
                            span.end -= deleted_len;
                        } else {
                            // Span partially overlaps deletion
                            span.start = span.start.min(deletion_range.start);
                            span.end = span.end.min(deletion_range.start);
                        }
                        span.start < span.end // Only keep if non-empty
                    }
                    SpanEditAction::Remove => false,
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ApplyAttributeError, AttributedText, EditBehavior, SpanEditAction};
    use alloc::string::String;
    use alloc::vec;

    #[derive(Debug, PartialEq)]
    enum TestAttribute {
        Keep,
        Remove,
    }

    impl EditBehavior for TestAttribute {
        fn on_edit(&self) -> SpanEditAction {
            match self {
                Self::Keep => SpanEditAction::Keep,
                Self::Remove => SpanEditAction::Remove,
            }
        }
    }

    #[test]
    fn bad_range_for_apply_attribute() {
        let t = "Hello!";
        let mut at = AttributedText::new(t);

        assert_eq!(at.apply_attribute(0..3, TestAttribute::Keep), Ok(()));
        assert_eq!(at.apply_attribute(0..6, TestAttribute::Keep), Ok(()));
        assert_eq!(
            at.apply_attribute(0..7, TestAttribute::Keep),
            Err(ApplyAttributeError::InvalidBounds)
        );
        assert_eq!(
            at.apply_attribute(7..8, TestAttribute::Keep),
            Err(ApplyAttributeError::InvalidBounds)
        );
    }

    #[test]
    fn test_delete_basic() {
        let mut text = AttributedText::new(String::from("Hello World"));
        text.apply_attribute(0..11, TestAttribute::Keep).unwrap();

        text.delete(5..11).unwrap(); // Delete " World"

        assert_eq!(text.text, "Hello");
        assert_eq!(text.attributes, vec![(0..5, TestAttribute::Keep)]);
    }

    #[test]
    fn test_delete_overlapping_remove() {
        let mut text = AttributedText::new(String::from("Hello World"));
        text.apply_attribute(0..11, TestAttribute::Remove).unwrap();

        text.delete(5..6).unwrap(); // Delete " "

        assert_eq!(text.text, "HelloWorld");
        assert!(text.attributes.is_empty());
    }
}
