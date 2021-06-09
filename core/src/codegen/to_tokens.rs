use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned as _,
    token,
};

use crate::{
    parse::{
        attrs::{dedup, field::TryMerge as _, kind},
        err,
        ext::ParseBuffer as _,
    },
    ParseAttrs,
};

/// Name of the derived trait.
const TRAIT_NAME: &str = "ToTokens";

/// Name of the helper attribute of this `proc_macro_derive`.
const ATTR_NAME: &str = "to_tokens";

pub fn derive(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    if !matches!(&input.data, syn::Data::Enum(_) | syn::Data::Struct(_)) {
        return Err(syn::Error::new(
            input.span(),
            format!("Only structs and enums can derive {}", TRAIT_NAME),
        ));
    }

    let attrs = Attrs::parse_attrs(ATTR_NAME, &input)?;

    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let impls = attrs.append.iter().map(|method| {
        quote! {
            ::synthez::quote::ToTokens::to_tokens(&self.#method(), out);
        }
    });

    Ok(quote! {
        #[automatically_derived]
        impl#impl_generics ::synthez::quote::ToTokens for #ty#ty_generics
            #where_clause
        {
            fn to_tokens(
                &self,
                out: &mut ::synthez::proc_macro2::TokenStream,
            ) {
                #( #impls )*
            }
        }
    })
}

#[derive(Debug, Default)]
struct Attrs {
    // #[parse(value)]
    append: Vec<syn::Ident>,
}

impl Parse for Attrs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut out = Self::default();
        while !input.is_empty() {
            let ident = input.fork().parse_any_ident()?;
            match ident.to_string().as_str() {
                "append" => {
                    input.skip_any_ident()?;
                    for v in input.parse_eq_or_wrapped_and_punctuated::<
                        syn::Ident, token::Paren, token::Comma,
                    >()? {
                        out.append.try_merge::<kind::Value, dedup::Unique>(v)?;
                    }
                }
                name => {
                    return Err(err::unknown_attr_arg(&ident, name));
                }
            }
            if input.try_parse::<token::Comma>()?.is_none() && !input.is_empty()
            {
                return Err(err::expected_followed_by_comma(&ident));
            }
        }
        Ok(out)
    }
}

impl ParseAttrs for Attrs {
    fn try_merge(mut self, another: Self) -> syn::Result<Self> {
        self.append
            .try_merge_self::<kind::Value, dedup::Unique>(another.append)?;
        Ok(self)
    }

    fn validate(&self, attr_name: &str, item_span: Span) -> syn::Result<()> {
        if self.append.is_empty() {
            return Err(syn::Error::new(
                item_span,
                format!(
                    "`#[{}(append(<function>))]` attribute is expected",
                    attr_name,
                ),
            ));
        }
        Ok(())
    }
}
