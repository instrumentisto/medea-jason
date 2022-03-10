//! `#[derive(Caused)]` macro implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Error, Result};
use synstructure::{BindStyle, Structure};

/// Generates the actual code for `#[derive(Caused)]` macro.
///
/// # Algorithm
///
/// 1. Generate body for trait method `name()` as enum variant name "as is".
/// 2. Generate body for trait method `cause()`:
///     - if `enum` variant contains associated error, returns this error;
///     - if `enum` variant contains `Caused`, invoke its trait method;
///     - otherwise returns `None`.
/// 3. Generate implementation of `Caused` trait for this enum with generated
///    methods from step 1 and 2.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn derive(mut s: Structure<'_>) -> Result<TokenStream> {
    let error_type = error_type(&s)?;

    let cause_body = s.bind_with(|_| BindStyle::Move).each_variant(|v| {
        v.bindings().iter().find(|&bi| is_caused(bi)).map_or_else(
            || {
                v.bindings()
                    .iter()
                    .find(|&bi| is_error(bi, &error_type))
                    .map_or_else(
                        || quote! { return None },
                        |err| quote! { return Some(#err) },
                    )
            },
            |caused| quote! { return #caused.cause() },
        )
    });

    let caused = s.gen_impl(quote! {
        #[automatically_derived]
        gen impl Caused for @Self {
            type Error = #error_type;

            fn cause(self) -> Option<Self::Error> {
                match self { #cause_body }
            }
        }
    });

    Ok(quote! { #caused })
}

/// Parse and returns argument of `#[cause(error = "path::to::Error"))]`
/// attribute. If no such attribute exists the defaults to `Error`.
fn error_type(s: &Structure<'_>) -> Result<syn::Path> {
    let mut error_type = None;
    for attr in &s.ast().attrs {
        if let Ok(meta) = attr.parse_meta() {
            if meta.path().is_ident("cause") {
                if error_type.is_some() {
                    return Err(Error::new_spanned(
                        meta,
                        "Cannot have two #[cause(...)] attributes",
                    ));
                }
                if let syn::Meta::List(list) = meta {
                    if list.nested.is_empty() {
                        return Err(Error::new_spanned(
                            list,
                            "Expected at least one argument to #[cause(...)] \
                             attribute",
                        ));
                    }
                    error_type = match &list.nested[0] {
                        syn::NestedMeta::Meta(syn::Meta::NameValue(nv))
                            if nv.path.is_ident("error") =>
                        {
                            if let syn::MetaNameValue {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            } = nv
                            {
                                Some(
                                    lit_str.parse_with(
                                        syn::Path::parse_mod_style,
                                    )?,
                                )
                            } else {
                                return Err(Error::new_spanned(
                                    nv,
                                    "Expected `path::to::error`",
                                ));
                            }
                        }
                        syn::NestedMeta::Meta(_) | syn::NestedMeta::Lit(_) => {
                            return Err(Error::new_spanned(
                                list,
                                "Expected attribute like #[cause(error = \
                                 \"path::to::error\")]",
                            ));
                        }
                    };
                } else {
                    return Err(Error::new_spanned(
                        meta,
                        "#[cause] attribute must take a list in parentheses",
                    ));
                };
            }
        }
    }
    match error_type {
        Some(path) => Ok(path),
        None => Err(Error::new_spanned(s.ast(), "Error type wasn't provided")),
    }
}

/// Checks that enum variant has `#[cause]` attribute.
fn is_caused(bi: &synstructure::BindingInfo<'_>) -> bool {
    let mut found_cause = false;
    for attr in &bi.ast().attrs {
        if let Ok(syn::Meta::Path(path)) = attr.parse_meta() {
            if path.is_ident("cause") {
                found_cause = true;
            }
        }
    }
    found_cause
}

/// Checks that enum variant contains JS error.
fn is_error(bi: &synstructure::BindingInfo<'_>, err: &syn::Path) -> bool {
    if let syn::Type::Path(syn::TypePath { qself: None, path }) = &bi.ast().ty {
        path == err
    } else {
        false
    }
}
