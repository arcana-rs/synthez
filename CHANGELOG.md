`synthez` changelog
===================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.1.0] · 2021-06-25
[0.1.0]: /../../tree/v0.1.0

### Initially implemented 

- `ParseAttrs` trait and derive macro for parsing `syn::Attribute`s in declarative way.
- Primitive `ToTokens` derive macro supporting only `#[to_tokens(append(<method>))]` attribute.
- `parse:attr::doc()`/`parse:attr::doc_string()` helpers for convenient parsing normalized Rust doc comments and `#[doc]` attributes.
- `Spanning` wrapper for attaching `Span` to arbitrary types.
- `Required` container for denoting `ParseAttrs` fields required to be provided.
- `IntoSpan` coercion working for both `Span` and `Spanned` types at the same time.
- `has::Attrs` trait abstracting `syn` types which contain `syn::Attribute`s.
- Extensions:
    - `IdentExt` simplifying `syn::Ident` creation;
    - `DataExt` simplifying `syn::Data` fields usage;
    - `ParseBufferExt` providing batteries for parsing `syn::Ident` and `syn::punctuated`.





[Semantic Versioning 2.0.0]: https://semver.org