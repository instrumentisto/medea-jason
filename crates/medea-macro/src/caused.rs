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
pub(crate) fn derive(s: &mut Structure<'_>) -> Result<TokenStream> {
    let error_type = error_type(s)?;

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

/// Parse and returns argument of `#[cause(error = path::to::Error))]`
/// attribute. If no such attribute exists the defaults to `Error`.
fn error_type(s: &Structure<'_>) -> Result<syn::Path> {
    let mut error_type = None;
    for attr in &s.ast().attrs {
        if attr.meta.path().is_ident("cause") {
            if error_type.is_some() {
                return Err(Error::new_spanned(
                    attr,
                    "Cannot have two #[cause(...)] attributes",
                ));
            }
            let syn::Meta::List(list) = &attr.meta else {
                return Err(Error::new_spanned(
                    &attr.meta,
                    "#[cause] attribute must take a list in parentheses",
                ));
            };
            let nv = syn::parse2::<syn::MetaNameValue>(list.tokens.clone())
                .map_err(|e| {
                    Error::new_spanned(
                        &list.tokens,
                        format!(
                            "Expected attribute like \
                             #[cause(error = path::to::error)], but: {e}",
                        ),
                    )
                })?;
            if !nv.path.is_ident("error") {
                return Err(Error::new_spanned(
                    &list.tokens,
                    "Expected attribute like #[cause(error = path::to::error)]",
                ));
            }
            let syn::Expr::Path(expr) = nv.value else {
                return Err(Error::new_spanned(
                    &nv.value,
                    "Expected `path::to::error`",
                ));
            };
            error_type = Some(expr.path);
        }
    }
    error_type.ok_or_else(|| {
        Error::new_spanned(s.ast(), "Error type wasn't provided")
    })
}

/// Checks that enum variant has `#[cause]` attribute.
fn is_caused(bi: &synstructure::BindingInfo<'_>) -> bool {
    for attr in &bi.ast().attrs {
        if let syn::Meta::Path(path) = &attr.meta {
            if path.is_ident("cause") {
                return true;
            }
        }
    }
    false
}

/// Checks that enum variant contains JS error.
fn is_error(bi: &synstructure::BindingInfo<'_>, err: &syn::Path) -> bool {
    if let syn::Type::Path(syn::TypePath { qself: None, path }) = &bi.ast().ty {
        path == err
    } else {
        false
    }
}
