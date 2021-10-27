#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::as_conversions,
    clippy::branches_sharing_code,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::else_if_without_else,
    clippy::empty_line_after_outer_attr,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::panic_in_result_fn,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trivial_regex,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    future_incompatible,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

// For correct Rust docs rendering only!
// TODO: Remove once docs render correctly without it.
#[allow(unused_imports)]
use syn as _;

#[doc(inline)]
pub use synthez_codegen::ToTokens;
#[doc(inline)]
pub use synthez_core::{ext, field, has, spanned};
pub use synthez_core::{
    proc_macro2,
    quote::{self, ToTokens},
    syn,
};

#[doc(inline)]
pub use self::{
    ext::{Data as DataExt, Ident as IdentExt},
    field::Required,
    parse::{Attrs as ParseAttrs, BufferExt as ParseBufferExt},
    spanned::Spanning,
};

pub mod parse {
    //! Batteries for [`syn::parse`](mod@crate::syn::parse).

    #[doc(inline)]
    pub use synthez_core::parse::{attr, err, ext};

    #[doc(inline)]
    pub use self::{attrs::Attrs, ext::ParseBuffer as BufferExt};

    pub mod attrs {
        //! Machinery for parsing [`syn::Attribute`]s into a custom defined
        //! struct.
        //!
        //! [`syn::Attribute`]: crate::syn::Attribute

        #[doc(inline)]
        pub use synthez_codegen::ParseAttrs as Attrs;
        #[doc(inline)]
        pub use synthez_core::parse::attrs::*;
    }
}
