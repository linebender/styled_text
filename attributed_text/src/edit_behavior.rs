// Copyright 2025 the Styled Text Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Trait to define edit behavior for attributes.
pub trait EditBehavior {
    /// Decide whether to keep or remove an attribute
    /// when the content has been edited.
    ///
    /// Defaults to [`SpanEditAction::Keep`].
    fn on_edit(&self) -> SpanEditAction {
        SpanEditAction::Keep
    }
}

/// Result of handling an edit for a span.
///
/// Returned from [`EditBehavior::on_edit`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpanEditAction {
    /// When the content of this span is edited, keep the span.
    ///
    /// This is typical of most attributes, especially style-oriented
    /// attributes.
    ///
    /// This is the default value.
    #[default]
    Keep,
    /// When the content of this span is edited, remove the span.
    ///
    /// This is typical for when the attribute carries a meaning
    /// that depends on the value of the text within the span,
    /// like indicators for spelling errors or compiler feedback
    /// for source code.
    Remove,
}
