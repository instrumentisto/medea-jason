//! `#[dispatchable]` macro implementation.

use inflector::Inflector as _;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens as _, quote};
use syn::{
    FnArg, ItemEnum, Pat, PatIdent, Token,
    parse::{Parse, ParseStream, Result},
    parse_quote, token,
};

/// Additional keywords to be parsed by [`syn`].
mod kw {
    syn::custom_keyword!(async_trait);
    syn::custom_keyword!(Send);
}

/// Generates the actual code for `#[dispatchable]` macro.
///
/// # Algorithm
///
/// 1. Generate dispatching `match`-arms for each `enum` variant.
/// 2. Generate trait methods signatures by transforming `enum` variant name
///    from `camelCase` to `snake_case` and add `on_` prefix.
/// 3. Generate trait `{enum_name}Handler` with generated methods from step 1.
/// 4. Generate method `dispatch_with()` with a dispatching generated on step 2.
pub(crate) fn expand(item: Item, args: &Args) -> TokenStream {
    let enum_ident = item.orig_enum.ident.clone();

    let dispatch_variants: Vec<_> = item
        .orig_enum
        .variants
        .iter()
        .map(|v| {
            let variant_ident = v.ident.clone();
            let handler_fn_ident = syn::Ident::new(
                &to_handler_fn_name(&variant_ident.to_string()),
                Span::call_site(),
            );
            let fields: &Vec<_> = &v
                .fields
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    f.ident.clone().unwrap_or_else(|| {
                        syn::Ident::new(&format!("f{i}"), Span::call_site())
                    })
                })
                .collect();
            match v.fields {
                syn::Fields::Named(_) => quote! {
                    #enum_ident::#variant_ident {#(#fields),*} => {
                        handler.#handler_fn_ident(#(#fields),*)
                    },
                },
                syn::Fields::Unnamed(_) => quote! {
                    #enum_ident::#variant_ident(#(#fields),*) => {
                        handler.#handler_fn_ident((#(#fields),*))
                    },
                },
                syn::Fields::Unit => quote! {
                    #enum_ident::#variant_ident => handler.#handler_fn_ident(),
                },
            }
        })
        .collect();

    let handler_kind = args.dispatch_with_handler_arg();
    let method_doc = item.dispatch_with_method_doc();
    let handler_trait = item.handler_trait(args);
    let maybe_async = args.maybe_async_token();
    let maybe_await = args.maybe_await_token();
    let orig_enum = item.orig_enum;
    let handler_trait_ident = item.handler_trait_ident;
    TokenStream::from(quote! {
        #orig_enum

        #handler_trait

        #[automatically_derived]
        impl #enum_ident {
            #[doc = #method_doc]
            pub #maybe_async fn dispatch_with<T: #handler_trait_ident>(
                self, #handler_kind,
            ) -> <T as #handler_trait_ident>::Output {
                match self {
                    #(#dispatch_variants)*
                }#maybe_await
            }
        }
    })
}

/// [`ItemEnum`] that `#[dispatchable]` macro is applied to, plus some misc
/// helpers.
#[derive(Debug)]
pub(crate) struct Item {
    /// Original enum definition to be dispatched.
    orig_enum: ItemEnum,

    /// `Handler` trait ident, basically `{}Handler` where `{}` is an enum
    /// name.
    handler_trait_ident: syn::Ident,
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let orig_enum = ItemEnum::parse(input)?;
        let handler_trait_ident = syn::Ident::new(
            &format!("{}Handler", orig_enum.ident),
            Span::call_site(),
        );
        Ok(Self { orig_enum, handler_trait_ident })
    }
}

impl Item {
    /// Returns `*Handler` trait documentation.
    fn handler_trait_doc(&self) -> String {
        format!(
            "Handler of [`{0}`] variants.\n\nUsing [`{0}::dispatch_with`] \
             method dispatches [`{0}`] variants to appropriate methods of \
             this trait.",
            self.orig_enum.ident
        )
    }

    /// Returns `dispatch_with` function documentation.
    fn dispatch_with_method_doc(&self) -> String {
        format!(
            "Dispatches [`{0}`] with given [`{0}Handler`].",
            self.orig_enum.ident
        )
    }

    /// Returns `*Handler` trait based on enum variants.
    fn handler_trait(&self, args: &Args) -> TokenStream2 {
        let self_kind = args.self_kind.clone();
        let maybe_async = args.maybe_async_token();
        let handler_trait_methods: Vec<_> = self
            .orig_enum
            .variants
            .iter()
            .map(|v| {
                let fn_name_ident = syn::Ident::new(
                    &to_handler_fn_name(&v.ident.to_string()),
                    Span::call_site(),
                );
                let handler_fn_args = match &v.fields {
                    syn::Fields::Named(fields) => {
                        let handler_fn_args: Vec<_> = fields
                            .named
                            .iter()
                            .filter_map(|f| {
                                let ident = f.ident.as_ref()?;
                                let ty = &f.ty;
                                Some(quote! { #ident: #ty })
                            })
                            .collect();
                        quote! { #(#handler_fn_args),* }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let handler_fn_args: Vec<_> = fields
                            .unnamed
                            .iter()
                            .map(|f| f.ty.clone())
                            .collect();
                        quote! { data: (#(#handler_fn_args),*) }
                    }
                    syn::Fields::Unit => quote! {},
                };
                let doc = format!(
                    "Handles [`{0}::{1}`] variant of [`{0}`].",
                    self.orig_enum.ident, v.ident,
                );

                quote! {
                    #[doc = #doc]
                    #maybe_async fn #fn_name_ident(
                        #self_kind,
                        #handler_fn_args
                    ) -> Self::Output;
                }
            })
            .collect();

        let trait_doc = self.handler_trait_doc();
        let handler_trait_ident = self.handler_trait_ident.clone();
        let maybe_async_trait_macro = args.maybe_async_trait_macro();
        let vis = self.orig_enum.vis.clone();
        quote! {
            #[automatically_derived]
            #[allow(clippy::needless_arbitrary_self_type)]
            #[doc = #trait_doc]
            #maybe_async_trait_macro
            #vis trait #handler_trait_ident {
                /// Output type of all functions from this trait.
                type Output;

                #(#handler_trait_methods)*
            }
        }
    }
}

/// [`async_trait`] configuration.
///
/// `false` is `#[async_trait]`, and `true` is `#[async_trait(?Send)]`.
///
/// [`async_trait`]: https://docs.rs/async-trait
#[derive(Debug, PartialEq)]
struct IsLocal(bool);

impl Parse for IsLocal {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.is_empty() {
            Ok(Self(false))
        } else {
            let inner;
            syn::parenthesized!(inner in input);
            _ = inner.parse::<Token![?]>()?;
            _ = inner.parse::<kw::Send>()?;
            Ok(Self(true))
        }
    }
}

/// Arguments of `#[dispatchable]` attribute.
#[derive(Debug, PartialEq)]
pub(crate) struct Args {
    /// `self` type that will be consumed by handler trait functions.
    self_kind: syn::Receiver,

    /// Whether to use [`async_trait`](https://crates.io/crates/async-trait)
    /// or not.
    async_trait: Option<IsLocal>,
}

impl Args {
    /// Generates `#[async_trait]` attribute depending of whether it's required
    /// by these [`Args`].
    fn maybe_async_trait_macro(&self) -> Option<TokenStream2> {
        self.async_trait.as_ref().map(|is_local| {
            if is_local.0 {
                quote! {
                    #[async_trait::async_trait(?Send)]
                }
            } else {
                quote! {
                    #[async_trait::async_trait]
                }
            }
        })
    }

    /// Generates `.await` token depending of whether it's required by these
    /// [`Args`].
    fn maybe_await_token(&self) -> Option<TokenStream2> {
        self.async_trait.as_ref().map(|_| quote! { .await })
    }

    /// Generates `async` token depending of whether it's required by these
    /// [`Args`].
    fn maybe_async_token(&self) -> Option<TokenStream2> {
        self.async_trait.as_ref().map(|_| quote! { async })
    }

    /// Transforms `self: &mut Self` to `handler: &mut T`.
    fn dispatch_with_handler_arg(&self) -> FnArg {
        let handler_arg = syn::PatType {
            attrs: vec![],
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: syn::Ident::new("handler", Span::call_site()),
                subpat: None,
            })),
            colon_token: token::Colon::default(),
            ty: self.self_kind.ty.clone(),
        };
        let handler_arg: TokenStream2 = handler_arg
            .to_token_stream()
            .into_iter()
            .map(|token| {
                if let TokenTree::Ident(ident) = &token {
                    if *ident == "Self" {
                        return TokenTree::Ident(syn::Ident::new(
                            "T",
                            ident.span(),
                        ));
                    }
                }
                token
            })
            .collect();
        parse_quote! { #handler_arg }
    }
}

/// Defaults are: `Args {self_kind: "self: &mut Self", async_trait: None}`.
impl Default for Args {
    fn default() -> Self {
        let self_kind = parse_quote! { self: &mut Self };
        let self_kind = match self_kind {
            FnArg::Receiver(rcv) => rcv,
            FnArg::Typed(_) => unreachable!(),
        };
        Self { self_kind, async_trait: None }
    }
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut args = Self::default();
        if input.is_empty() {
            return Ok(args);
        }

        if input.peek(Token![self]) && input.peek2(Token![:]) {
            let self_kind = FnArg::parse(input)?;
            let self_kind = match self_kind {
                FnArg::Receiver(rcv) => rcv,
                FnArg::Typed(_) => {
                    return Err(syn::Error::new_spanned(
                        self_kind,
                        "Invalid argument",
                    ));
                }
            };
            args.self_kind = self_kind;
        }
        if input.peek(Token![,]) {
            _ = input.parse::<Token![,]>()?;
        }
        if input.peek(kw::async_trait) {
            _ = input.parse::<kw::async_trait>()?;
            args.async_trait = Some(IsLocal::parse(input)?);
        }

        Ok(args)
    }
}

/// Transforms given name from `camelCase` to `snake_case` and adds `on_`
/// prefix.
fn to_handler_fn_name(name: &str) -> String {
    let mut snake_case = name.to_snake_case();
    snake_case.insert_str(0, "on_");
    snake_case
}

#[cfg(test)]
mod to_handler_fn_name_spec {
    use super::*;

    #[test]
    fn converts_name_from_camel_case_to_snake_case() {
        for (name, expected) in [
            ("SomeTestTrait", "on_some_test_trait"),
            ("RPCConnection", "on_rpc_connection"),
            ("RConnection", "on_r_connection"),
            ("RTCPeerConnection", "on_rtc_peer_connection"),
            ("testString", "on_test_string"),
            ("testtest", "on_testtest"),
            ("Some", "on_some"),
            ("S", "on_s"),
            ("s", "on_s"),
            ("ASDF", "on_asdf"),
        ] {
            assert_eq!(to_handler_fn_name(name), expected);
        }
    }

    mod parse_args {
        use syn::parse::Parser as _;

        use super::*;

        #[test]
        fn empty() {
            let args = Args::parse.parse2(quote! {}).unwrap();
            assert_eq!(
                args.dispatch_with_handler_arg(),
                FnArg::parse.parse2(quote! {handler: &mut T}).unwrap(),
            );
            assert!(args.async_trait.is_none());
            assert_eq!(
                FnArg::Receiver(args.self_kind),
                FnArg::parse.parse2(quote! {self: &mut Self}).unwrap(),
            );
        }

        #[test]
        fn self_ref() {
            let args = Args::parse.parse2(quote! {self: &Self}).unwrap();
            assert_eq!(
                args.dispatch_with_handler_arg(),
                FnArg::parse.parse2(quote! {handler: &T}).unwrap(),
            );
            assert!(args.async_trait.is_none());
            assert_eq!(
                FnArg::Receiver(args.self_kind),
                FnArg::parse.parse2(quote! { self: &Self }).unwrap(),
            );
        }

        #[test]
        fn self_rc() {
            let args =
                Args::parse.parse2(quote! {self: std::rc::Rc<Self>}).unwrap();
            assert_eq!(
                args.dispatch_with_handler_arg(),
                FnArg::parse.parse2(quote! {handler: std::rc::Rc<T>}).unwrap(),
            );
            assert!(args.async_trait.is_none());
            assert_eq!(
                FnArg::Receiver(args.self_kind),
                FnArg::parse.parse2(quote! {self: std::rc::Rc<Self>}).unwrap(),
            );
        }

        #[test]
        fn async_trait_not_local() {
            let args = Args::parse.parse2(quote! {async_trait}).unwrap();
            assert_eq!(
                args.dispatch_with_handler_arg(),
                FnArg::parse.parse2(quote! {handler: &mut T}).unwrap(),
            );
            assert!(!args.async_trait.unwrap().0);
            assert_eq!(
                FnArg::Receiver(args.self_kind),
                FnArg::parse.parse2(quote! {self: &mut Self}).unwrap(),
            );
        }

        #[test]
        fn async_trait_local() {
            let args = Args::parse.parse2(quote! {async_trait(?Send)}).unwrap();
            assert_eq!(
                args.dispatch_with_handler_arg(),
                FnArg::parse.parse2(quote! {handler: &mut T}).unwrap(),
            );
            assert!(args.async_trait.unwrap().0);
            assert_eq!(
                FnArg::Receiver(args.self_kind),
                FnArg::parse.parse2(quote! {self: &mut Self}).unwrap(),
            );
        }

        #[test]
        fn self_arc_and_async_trait_not_send() {
            let args = Args::parse
                .parse2(quote! {self: Arc<Self>, async_trait})
                .unwrap();
            assert_eq!(
                args.dispatch_with_handler_arg(),
                FnArg::parse.parse2(quote! {handler: Arc<T>}).unwrap(),
            );
            assert!(!args.async_trait.unwrap().0);
            assert_eq!(
                FnArg::Receiver(args.self_kind),
                FnArg::parse.parse2(quote! {self: Arc<Self>}).unwrap(),
            );
        }
    }
}
