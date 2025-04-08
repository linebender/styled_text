// Copyright 2025 the Styled Text Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::string::String;
use alloc::sync::Arc;
use core::ops::Range;

/// A block of text that will be wrapped by an [`AttributedText`].
///
/// [`AttributedText`]: crate::AttributedText
pub trait TextStorage {
    /// The length of the underlying text.
    fn len(&self) -> usize;

    /// Return `true` if the underlying text is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A block of text that can be edited that will be wrapped by an [`AttributedText`].
///
/// This enables editing operations on [`AttributedText`].
///
/// [`AttributedText`]: crate::AttributedText
pub trait EditableTextStorage: TextStorage {
    /// Removes the specified range in the text, and replaces it with the specified text.
    ///
    /// The specified text doesn't need to the same length as the range.
    fn replace_range(&mut self, range: Range<usize>, replacement_text: &str);
}

impl TextStorage for String {
    fn len(&self) -> usize {
        Self::len(self)
    }
}

impl TextStorage for &str {
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl TextStorage for Arc<str> {
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl EditableTextStorage for String {
    fn replace_range(&mut self, range: Range<usize>, replacement_text: &str) {
        Self::replace_range(self, range, replacement_text);
    }
}
