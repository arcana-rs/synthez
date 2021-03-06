//! Common errors of parsing.

use crate::spanned::IntoSpan;

/// Creates a "duplicated attribute's argument" [`syn::Error`] pointing to the
/// given [`Span`].
///
/// [`Span`]: proc_macro2::Span
#[must_use]
pub fn dup_attr_arg<S: IntoSpan>(span: S) -> syn::Error {
    syn::Error::new(span.into_span(), "duplicated attribute's argument found")
}

/// Creates an "unknown attribute's argument" [`syn::Error`] for the given
/// `name` pointing to the given [`Span`].
///
/// [`Span`]: proc_macro2::Span
#[must_use]
pub fn unknown_attr_arg<S: IntoSpan>(span: S, name: &str) -> syn::Error {
    syn::Error::new(
        span.into_span(),
        // TODO: Use "{name}" syntax once MSRV bumps above 1.58.
        format!("unknown `{}` attribute argument", name),
    )
}

/// Creates an "expected followed by comma" [`syn::Error`] in the given
/// [`Span`].
///
/// [`Span`]: proc_macro2::Span
#[must_use]
pub fn expected_followed_by_comma<S: IntoSpan>(span: S) -> syn::Error {
    syn::Error::new(span.into_span(), "expected followed by `,`")
}
