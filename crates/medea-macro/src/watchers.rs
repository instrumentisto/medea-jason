//! `#[watchers]` macro implementation.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ExprMethodCall, ImplItem, ItemImpl,
    parse::{Error, Result},
};

/// Generates the actual code for `#[watchers]` macro.
///
/// # Algorithm
///
/// 1. Collects all methods with a `#[watch(...)]` attribute.
///
/// 2. Removes `#[watch(...)]` attributes from the found methods.
///
/// 3. Generates `WatchersSpawner::spawn()` code for the found methods.
///
/// 4. Generates `ComponentState` implementation with all the generated
///    `WatchersSpawner::spawn()` method calls.
///
/// 5. Appends the generated `ComponentState` implementation to the input.
pub(crate) fn expand(mut input: ItemImpl) -> Result<TokenStream> {
    let component_ty = input.self_ty.clone();

    let watchers = input
        .items
        .iter_mut()
        .filter_map(|i| if let ImplItem::Fn(m) = i { Some(m) } else { None })
        .map(|method| {
            let mut watch_attr_index = None;
            let stream_expr = method
                .attrs
                .iter()
                .enumerate()
                .find_map(|(i, attr)| {
                    attr.path().get_ident().is_some_and(|p| *p == "watch").then(
                        || {
                            watch_attr_index = Some(i);
                            attr
                        },
                    )
                })
                .ok_or_else(|| {
                    Error::new(
                        method.sig.ident.span(),
                        "Method doesn't have '#[watch(...)]' macro",
                    )
                })?
                .parse_args::<ExprMethodCall>()?;
            if let Some(index) = watch_attr_index {
                drop(method.attrs.remove(index));
            }

            let watcher_ident = &method.sig.ident;

            let spawner = if method.sig.asyncness.is_some() {
                quote! { spawn }
            } else {
                quote! { spawn_sync }
            };

            Ok(quote! {
                s.#spawner(#stream_expr, #component_ty::#watcher_ident);
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let component = quote! {
        <#component_ty as crate::utils::component::ComponentTypes>
    };

    Ok(quote! {
        #[allow(
            clippy::let_underscore_untyped,
            clippy::multiple_inherent_impl,
            let_underscore_drop,
        )]
        #input

        impl crate::utils::component::ComponentState<
            #component::Obj,
        > for #component::State {
            fn spawn_watchers(
                &self,
                s: &mut crate::utils::component::WatchersSpawner<
                    Self,
                    #component::Obj,
                >,
            ) {
                #( #watchers )*
            }
        }
    }
    .into())
}
